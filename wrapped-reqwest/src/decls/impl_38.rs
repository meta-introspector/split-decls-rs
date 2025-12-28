macro_rules! deps {
    () => {
        IntoUrl!();
    };
}

macro_rules! impl_38 {
    () => {
        deps!();
        impl IntoUrl for String { }
    };
}

impl_38!()