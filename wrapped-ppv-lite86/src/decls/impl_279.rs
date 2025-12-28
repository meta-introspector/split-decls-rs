macro_rules! deps {
    () => {
        RotateEachWord128!();
    };
}

macro_rules! impl_279 {
    () => {
        deps!();
        impl RotateEachWord128 for u128x1_generic { }
    };
}

impl_279!()