macro_rules! deps {
    () => {
        RotateEachWord128!();
    };
}

macro_rules! impl_39 {
    () => {
        deps!();
        impl < W > RotateEachWord128 for x4 < W > where W : RotateEachWord128 { }
    };
}

impl_39!()