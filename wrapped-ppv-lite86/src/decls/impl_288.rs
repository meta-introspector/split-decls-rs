macro_rules! deps {
    () => {
        BitOps0!();
    };
}

macro_rules! impl_288 {
    () => {
        deps!();
        impl BitOps0 for u32x4_generic { }
    };
}

impl_288!();