macro_rules! deps {
    () => {
        OutOfRangeError!();
    };
}

macro_rules! impl_25 {
    () => {
        deps!();
        impl fmt :: Display for OutOfRangeError { fn fmt (& self , f : & mut fmt :: Formatter) -> fmt :: Result { write ! (f , "Source duration value is out of range for the target type") } }
    };
}

impl_25!();