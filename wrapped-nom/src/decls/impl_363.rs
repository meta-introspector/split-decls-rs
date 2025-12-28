macro_rules! deps {
    () => {
        ErrorConvert!();
        ErrorKind!();
    };
}

macro_rules! impl_363 {
    () => {
        deps!();
        impl < I > ErrorConvert < (I , ErrorKind) > for ((I , usize) , ErrorKind) { fn convert (self) -> (I , ErrorKind) { ((self . 0) . 0 , self . 1) } }
    };
}

impl_363!()