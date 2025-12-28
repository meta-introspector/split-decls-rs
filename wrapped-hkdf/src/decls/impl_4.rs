macro_rules! deps {
    () => {
        InvalidLength!();
    };
}

macro_rules! impl_4 {
    () => {
        deps!();
        impl fmt :: Display for InvalidLength { fn fmt (& self , f : & mut fmt :: Formatter < '_ >) -> Result < () , fmt :: Error > { f . write_str ("invalid number of blocks, too large output") } }
    };
}

impl_4!();