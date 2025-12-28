macro_rules! deps {
    () => {
        Error!();
    };
}

macro_rules! impl_10 {
    () => {
        deps!();
        impl fmt :: Display for Error { fn fmt (& self , f : & mut fmt :: Formatter < '_ >) -> Result < () , fmt :: Error > { f . write_str ("Block buffer error") } }
    };
}

impl_10!()