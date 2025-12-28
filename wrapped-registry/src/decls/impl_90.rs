macro_rules! deps {
    () => {
        Data!();
    };
}

macro_rules! impl_90 {
    () => {
        deps!();
        impl Eq for Data { }
    };
}

impl_90!()