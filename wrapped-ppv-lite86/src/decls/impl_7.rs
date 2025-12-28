macro_rules! deps {
    () => {
        RotateEachWord128!();
    };
}

macro_rules! impl_7 {
    () => {
        deps!();
        impl < W , G > RotateEachWord128 for x2 < W , G > where W : RotateEachWord128 { }
    };
}

impl_7!()