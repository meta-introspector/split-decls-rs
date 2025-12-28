macro_rules! deps {
    () => {
        ErrorConvert!();
        ErrorKind!();
    };
}

macro_rules! impl_364 {
    () => {
        deps!();
        impl < I > ErrorConvert < ((I , usize) , ErrorKind) > for (I , ErrorKind) { fn convert (self) -> ((I , usize) , ErrorKind) { ((self . 0 , 0) , self . 1) } }
    };
}

impl_364!();