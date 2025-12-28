macro_rules! deps {
    () => {
        BitOps32!();
    };
}

macro_rules! impl_285 {
    () => {
        deps!();
        impl BitOps32 for u32x4_generic { }
    };
}

impl_285!()