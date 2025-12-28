macro_rules! deps {
    () => {
        BitOps128!();
    };
}

macro_rules! impl_280 {
    () => {
        deps!();
        impl BitOps128 for u128x1_generic { }
    };
}

impl_280!();