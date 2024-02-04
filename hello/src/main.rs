use core::num;

fn main() {
    // let x:i8 = 12;
    // let y = 02_55;
    // let z:[i32;3] = [1,2,3];

    // let mut z= vec![1,2 ,3,];
    // z.push(4);
    
    // // println!("the value of x is {}",x);
    // // println!("the value of y is {}",y);
    // println!("the value of z is {:?}",z);
    // z.pop();
    // println!("the value of z is {:?}",z);

    // let mut y = Vec::new();
    // y.push("zohaib");
    // y.push("malik");
    // println!("the value of y is {:?}",y);

    // let mut x = Vec::<i32>::with_capacity(2);
    // println!("the value of x is {:?}",x.capacity());

    // let v:Vec<i32> = (0..5).collect();
    // println!("the value of v is {:?}",v);

    // let sv:&[i32] = &v;
    // println!("the value of sv is {:?}",sv);

    // let name = String::from("Zohaib");

    // let course = "rust".to_string();

    // let new_name = name.replace("Zohaib", "zabi");
    // println!("the value of z is {}",name);
    // println!("the value of z is {}",course);
    // println!("the value of z is {}",new_name);

    // let str = "hello";
    // let str2 = str.to_string();
    // let str3 = &&str2;
    // println!("the value of z is {}",str);
    // println!("the value of z is {}",str2);
    // println!("the value of z is {}",str3);

    // let x = "\x52\x72\x73";
    // println!("the value of z is {}",x);
    // hello_print("hii this is zohaib");
    // let a:u64 = gcd(23, 12);
    // println!("the value of z is {}",a);

    let mut counter = 0;
    'counters: loop {
        println!("{}",counter);
        let mut decrease = 5;
        loop {
            if decrease ==1 {
                break ;
            }
            if counter == 2 {
                break 'counters;
            }
            decrease -= 1;
        }
        counter += 1;
    }

    for number in (1..4).rev(){
        print!("{}",number);
    }
    
}

fn hello_print(pharse:&str){
    println!("{}",pharse);
}

fn gcd(mut a: u64,mut b: u64) -> u64 {
    while a != 0 {
        if a < b {
            let c = a;
            a = b;
            b= c;

        }
        a = a%b;

    }
    b
}