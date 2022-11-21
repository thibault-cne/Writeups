use std::fs;

fn main() {
	/* let mut file = File::open("/etc/passwd")
		.expect("Rien");
	let mut data = String::new();
	file.read_to_string(&mut data)
		.expect("Rien");
	println!("{}", data);
	*/
	let data = fs::read("./flag");
	println!("{:?}", data);
}

