macro_rules! deps {
    () => {
        Error!();
    };
}

macro_rules! impl_485 {
    () => {
        deps!();
        impl fmt :: Display for Error { fn fmt (& self , formatter : & mut fmt :: Formatter) -> Result < () , fmt :: Error > { self . message . fmt (formatter) } }
    };
}

impl_485!()