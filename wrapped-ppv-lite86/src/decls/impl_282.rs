macro_rules! deps {
    () => {
        BitOps64!();
    };
}

macro_rules! impl_282 {
    () => {
        deps!();
        impl BitOps64 for u64x2_generic { }
    };
}

impl_282!();