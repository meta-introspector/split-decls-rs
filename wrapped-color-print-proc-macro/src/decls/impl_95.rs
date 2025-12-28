macro_rules! deps {
    () => {
        ErrorDetail!();
        Error!();
    };
}

macro_rules! impl_95 {
    () => {
        deps!();
        impl < 'a > Error < 'a > { pub fn new (input : & 'a str , code : ErrorKind , detail : Option < ErrorDetail < 'a > >) -> Self { Error { input , code , detail } } pub fn with_detail (& self , detail : ErrorDetail < 'a >) -> Self { Error { input : self . input , code : self . code , detail : Some (detail) } } }
    };
}

impl_95!();