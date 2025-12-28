macro_rules! ConstBuffer {
    () => {
        pub struct ConstBuffer { data : [u8 ; BUFFER_SIZE] , head : usize , }
    };
}

ConstBuffer!()