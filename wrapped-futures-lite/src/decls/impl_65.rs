macro_rules! deps {
    () => {
        Repeat!();
    };
}

macro_rules! impl_65 {
    () => {
        deps!();
        impl < T > Unpin for Repeat < T > { }
    };
}

impl_65!()