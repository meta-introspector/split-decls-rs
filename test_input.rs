fn hello_world() {
    println!("Hello, world!");
}

struct MyStruct {
    field1: i32,
    field2: String,
}

enum MyEnum {
    Variant1,
    Variant2(i32),
    Variant3 { x: i32, y: i32 },
}

trait MyTrait {
    fn method1(&self) -> i32;
    fn method2(&mut self, value: i32);
}

impl MyTrait for MyStruct {
    fn method1(&self) -> i32 {
        self.field1
    }
    
    fn method2(&mut self, value: i32) {
        self.field1 = value;
    }
}
