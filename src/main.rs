// use std::thread::sleep;
// use std::time::Duration;

use std::collections::VecDeque;

fn test_vector_wahyu() {
    let mut data_one: Vec<&str> = vec!["batman", "superman", "lobo"];
    println!("data: {:?}", data_one);
    println!(
        "length: {}, capacity: {}",
        data_one.len(),
        data_one.capacity()
    );

    data_one.pop();

    println!("data: {:?}", data_one);
    println!(
        "length: {}, capacity: {}",
        data_one.len(),
        data_one.capacity()
    );

    data_one.remove(1);

    println!("data: {:?}", data_one);
    println!(
        "length: {}, capacity: {}",
        data_one.len(),
        data_one.capacity()
    );

    data_one.push("constantine");
    data_one.push("trigon");
    data_one.push("darkseid");

    println!("data: {:#?}", data_one);
    println!(
        "length: {}, capacity: {}",
        data_one.len(),
        data_one.capacity()
    );

    data_one[2] = "red hood";

    println!("data: {:#?}", data_one);
    println!(
        "length: {}, capacity: {}",
        data_one.len(),
        data_one.capacity()
    );

    let is_vector_empty = data_one.is_empty();
    println!("is vector empty: {:?}", is_vector_empty);

    data_one.clear();

    println!("data: {:?}", data_one);
    println!(
        "length: {}, capacity: {}",
        data_one.len(),
        data_one.capacity()
    );

    // append
    let mut result_one = vec![3, 1, 2];

    let mut data_two = vec![7, 6, 8];
    result_one.append(&mut data_two);

    println!("data: {:?}", result_one);
    println!(
        "length: {}, capacity: {}",
        result_one.len(),
        result_one.capacity()
    );

    result_one.append(&mut vec![4, 5]);

    println!("data: {:?}", result_one);
    println!(
        "length: {}, capacity: {}",
        result_one.len(),
        result_one.capacity()
    );

    // sorti
    println!("data: {:?}", result_one);
    result_one.sort();
    println!("data after sort: {:?}", result_one);

    // declare empty vector
    let vector_7: Vec<&str> = vec![];
    let vector_8: Vec<&str> = Vec::new();

    println!(
        "length: {}, capacity: {}",
        vector_7.len(),
        vector_7.capacity()
    );
    println!(
        "length: {}, capacity: {}",
        vector_8.len(),
        vector_8.capacity()
    );

    let vec_nine = vec![1, 2, 3];
    for i in 0..=vec_nine.len() - 1 {
        println!("wkwk {} ", vec_nine[i]);
    }

    let vec_ten = vec![1, 3, 2555];
    for e in vec_ten.iter() {
        print!("{e} ");
    }
    println!();
    for i in 0..vec_ten.len() {
        print!("{} ", vec_ten[i]);
    }
    println!();

    let vec_population = vec![2, 3, 4];
    let vec_sample = &vec_population[0..1];
    println!("{:?}", vec_sample);

    let mut vec_deq = VecDeque::from(vec!["a", "b", "c"]);

    vec_deq.pop_front();
    vec_deq.push_front("z");
    println!("data: {:?}", vec_deq);

    vec_deq.pop_back();
    vec_deq.push_back("h");
    println!("data: {:?}", vec_deq);
}
/*
fn test_array_wahyu() {
    let mut numbers: [i64; 4] = [12, 2312 ,232, 213];
    println!("array {:?}", numbers);

    let data0 = numbers[0];
    println!("elemen aray ke 0 {data0}");

    let data1 = numbers[2];
    println!("eleme ararya k2 {}", data1);

    numbers[1] = 99;
    println!("asdas baru {numbers:?}");

    println!("total arar: {}", numbers.len());
}

fn test_tuple_wahyu() {
    let texts = ["disini", "kau", "aku"];
    let tuple_a = ("wahyu", 2, &texts[1..], 6.7);
    println!("index ke 0: {:?}", tuple_a.0);
    println!("index ke 2: {:?}", tuple_a.2);
    println!("index ke 1: {:?}", tuple_a.1);
    println!("index ke 3: {:?}", tuple_a.3);
}


fn test_basic_wahyu () {
    println!("Hello, world!");

    let a = 12;
    println!("{}", -a == -12);

    let (value_left, value_right) = (12, -12);
    let res_one = -value_left == value_right;
    let res_two = !(value_left == value_right);
    println!("{res_one} {res_two}");

    let mut i = 0;
    let max = 5;

    while i < max {
        let mut j = 0;
        let max_inner = i;

        while j <= max_inner {
            print!("* ");
            j += 1;
        }

        println!();
        i += 1;
    }

    let mut x = 0;
    while x < max {
        println!("nilai x: {x}");
        x += 1;

        sleep(Duration::from_secs(1));
    }

    let mut l = 0;
    loop {
        println!("nilai l: {l}");
        l += 1;
        if l > 5 {
            break;
        }
    }

    let mut y: i8 = 0;
    let wewok= 5;
    loop {
        let mut z = wewok;
        let max_inner = y;

        loop {
            print!("* ");
            z -= 1;
            if z < max_inner {
                break;
            }
        }

        println!();

        y += 1;
        if y > max {
            break;
        }
    }

    let mut g = 0;
    let h = 15;
    loop {

        g += 1;

        if g % 2 == 1 {
            continue;
        }

        println!("nilai g sekarang : {g}");

        if g > h {
            break;
        }
    }

    // label
    let mut ja = 0;
    let k_max = 9;
    'mainLoop: loop {
        ja += 1;
        let mut ka = 0;

        loop {
            if ja > k_max {
                break 'mainLoop;
            }

            ka += 1;
            if ka > ja {
                break;
            }

            print!("{ka} ");


        }

        println!();


    }

    // return loop
    let mut counter = 0;
    let result = loop {
        counter += 1;

        if counter == 10 {
            break counter * 123;
        }
    };

    println!("result: {result}");

    for i in 1..4 {
        println!("{i}")
    }

    // label for in
    'perulangan: for i in 0..5 {

        if i == 2 {
            continue;
        }

        if i > 3 {
            println!("perulangan dihentikan paksa pada iterasi {i}");
            break 'perulangan;
        }

        println!("{i}")
    }
}
*/

fn main() {
    test_vector_wahyu();
}
