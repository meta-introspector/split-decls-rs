macro_rules! deps {
    () => {
        Float!();
    };
}

macro_rules! impl_372 {
    () => {
        deps!();
        impl Float for f64 { }
    };
}

impl_372!();