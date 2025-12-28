macro_rules! deps {
    () => {
        InvalidPrkLength!();
    };
}

macro_rules! impl_1 {
    () => {
        deps!();
        impl fmt :: Display for InvalidPrkLength { fn fmt (& self , f : & mut fmt :: Formatter < '_ >) -> Result < () , fmt :: Error > { f . write_str ("invalid pseudorandom key length, too short") } }
    };
}

impl_1!()