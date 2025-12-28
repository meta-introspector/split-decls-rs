macro_rules! deps {
    () => {
        BitOps64!();
    };
}

macro_rules! impl_281 {
    () => {
        deps!();
        impl BitOps64 for u128x1_generic { }
    };
}

impl_281!();