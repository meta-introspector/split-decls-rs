macro_rules! Data {
    () => {
        pub struct Data { ptr : * mut u8 , len : usize , }
    };
}

Data!()