macro_rules! deps {
    () => {
        IntoUrl!();
    };
}

macro_rules! impl_40 {
    () => {
        deps!();
        impl < 'a > IntoUrl for & 'a String { }
    };
}

impl_40!()