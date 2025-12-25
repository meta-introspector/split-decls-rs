pub fn hello() -> String {
    "Hello, World!".to_string()
}

pub fn add(a: i32, b: i32) -> i32 {
    a + b
}

pub struct TestStruct {
    pub value: i32,
}

impl TestStruct {
    pub fn new(value: i32) -> Self {
        Self { value }
    }
}

pub enum TestEnum {
    Variant1,
    Variant2(i32),
}
