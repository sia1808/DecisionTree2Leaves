use std::fs::File;
use std::io::prelude::*;
use rand::Rng;

fn generate_file(n:usize) {
    // Generate random file of edges for vertices 0 to n 
    let mut file = File::create("data.txt").expect("Unable to create file");
    for _i in 0..n {
        let rng = rand::thread_rng().gen_range(0..100);
        for _j in 0..rng {
            let one = rand::thread_rng().gen_range(-100000..=100000);
            let two = rand::thread_rng().gen_range(0..=1);
            file.write_all(format!("{} {}\n", one ,two).as_bytes()).expect("Error writing to file");
        }
    }
}

// Function to read data from the file into a vector of tuples
fn read_file() -> Vec<(isize, usize)> {
    let mut ds:Vec<(isize, usize)> = Vec::new();
    let file = File::open("data.txt").expect("Could not open file");
    let reader = std::io::BufReader::new(file).lines();
    for line in reader {
        let lines = line.expect("Error Reading");
        let i: Vec<&str> = lines.trim().split(" ").collect();
        let j = i[0].parse::<isize>().unwrap();
        let k = i[1].parse::<usize>().unwrap();
        ds.push((j,k));
    }
    return ds;
}

// Function to calculate accuracy given actual and predicted labels
fn accuracy(z: &Vec<usize>,pred: &Vec<usize>)-> f64{
    let mut sum = 0;
    for num in 0..(z.len()){
        if z[num] == pred[num]{
            sum += 1;
        }
    }

    let accuracy:f64 = sum as f64/z.len() as f64;
    return accuracy 
}

// Function to split the data at a given point and calculate accuracy
fn split(j: &Vec<isize>, z: &Vec<usize>, point:&isize) -> f64 {
    let mut predicted:Vec<usize> = Vec::new();
    for num in j {
        if num>=point{
            predicted.push(1);
        }else{
            predicted.push(0);
        }
    }
    return accuracy(&z, &predicted);
}

fn main() {
    generate_file(4); // Generate data file with 4 points
    let ds = read_file(); // Read data from file

    // Extract x and z values into separate vectors
    let (j, z): (Vec<isize>, Vec<usize>) = ds.iter().cloned().unzip();

    let mut accuracy_best = 0.0;
    let mut point_best = 0;

    // Iterate over each point as a potential split point
    for &point in &j {
        // Calculate accuracy for splitting at current point
        let accuracy = split(&j, &z, &point);

        // Update best accuracy and split point if current accuracy is higher
        if accuracy > accuracy_best {
            accuracy_best = accuracy;
            point_best = point;
        }
    }

    // Output decision tree and accuracy
    println!("if x >= {}", point_best);
    println!("   Predicted label is 1");
    println!("else");
    println!("   Predicted label is 0");
    println!("accuracy: {}.", accuracy_best);
}


#[test]
fn test_decision_tree() {
    // Generate a small test dataset with known outcomes
    let test_data = vec![
        (-15, 0), // Expect 0
        (15, 1),  // Expect 1
        (-5, 1),  // Expect 1
        (5, 0),   // Expect 0
    ];
    // Write the test data to a file
    {
        let mut file = File::create("test_data.txt").expect("Unable to create test data file");
        for &(x, z) in &test_data {
            writeln!(file, "{} {}", x, z).expect("Error writing to test data file");
        }
    }
    // Read the test data from the file
    let data = read_file("test_data.txt");
    // Extract x and z values into separate vectors
    let (j, z): (Vec<isize>, Vec<usize>) = data.iter().cloned().unzip();

    // Determine the decision tree
    let mut accuracy_best = 0.0;
    let mut point_best = 0;
    for &point in &j {
        let accuracy = split(&j, &z, &point);
        if accuracy > accuracy_best {
            accuracy_best = accuracy;
            point_best = point;
        }
    }
    // Assert the decision tree against expected values
    assert_eq!(point_best, 10);
    assert_eq!(accuracy_best, 0.75);
}
