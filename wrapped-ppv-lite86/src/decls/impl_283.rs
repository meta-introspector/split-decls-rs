macro_rules! deps {
    () => {
        BitOps32!();
    };
}

macro_rules! impl_283 {
    () => {
        deps!();
        impl BitOps32 for u128x1_generic { }
    };
}

impl_283!()