macro_rules! deps {
    () => {
        BitOps32!();
    };
}

macro_rules! impl_284 {
    () => {
        deps!();
        impl BitOps32 for u64x2_generic { }
    };
}

impl_284!()