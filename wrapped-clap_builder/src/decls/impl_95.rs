macro_rules! deps {
    () => {
        AppExt!();
        MaxTermWidth!();
    };
}

macro_rules! impl_95 {
    () => {
        deps!();
        impl AppExt for MaxTermWidth { }
    };
}

impl_95!();