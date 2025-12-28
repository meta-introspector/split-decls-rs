macro_rules! deps {
    () => {
        MaxTermWidth!();
        AppExt!();
    };
}

macro_rules! impl_95 {
    () => {
        deps!();
        impl AppExt for MaxTermWidth { }
    };
}

impl_95!()