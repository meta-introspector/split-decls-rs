macro_rules! deps {
    () => {
        InvalidLengthError!();
        Error!();
    };
}

macro_rules! impl_45 {
    () => {
        deps!();
        impl fmt :: Display for InvalidLengthError { fn fmt (& self , f : & mut fmt :: Formatter < '_ >) -> Result < () , fmt :: Error > { f . write_str (INVALID_LENGTH_MSG) } }
    };
}

impl_45!()