macro_rules! deps {
    () => {
        FixedBitSet!();
    };
}

macro_rules! impl_90 {
    () => {
        deps!();
        impl Display for FixedBitSet { fn fmt (& self , f : & mut Formatter < '_ >) -> Result < () , Error > { Binary :: fmt (& self , f) } }
    };
}

impl_90!();