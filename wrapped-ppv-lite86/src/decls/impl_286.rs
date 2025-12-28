macro_rules! deps {
    () => {
        BitOps0!();
    };
}

macro_rules! impl_286 {
    () => {
        deps!();
        impl BitOps0 for u128x1_generic { }
    };
}

impl_286!()