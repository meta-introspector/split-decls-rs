macro_rules! deps {
    () => {
        IntoUrl!();
    };
}

macro_rules! impl_39 {
    () => {
        deps!();
        impl < 'a > IntoUrl for & 'a str { }
    };
}

impl_39!();