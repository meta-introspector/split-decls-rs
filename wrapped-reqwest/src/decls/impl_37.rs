macro_rules! deps {
    () => {
        IntoUrl!();
    };
}

macro_rules! impl_37 {
    () => {
        deps!();
        impl IntoUrl for Url { }
    };
}

impl_37!();