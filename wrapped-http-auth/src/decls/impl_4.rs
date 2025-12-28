macro_rules! deps {
    () => {
        Error!();
    };
}

macro_rules! impl_4 {
    () => {
        deps!();
        impl < 'i > Error < 'i > { fn invalid_byte (input : & 'i str , pos : usize) -> Self { Self { input , pos , error : "invalid byte" , } } }
    };
}

impl_4!();