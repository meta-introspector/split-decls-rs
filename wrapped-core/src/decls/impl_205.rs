macro_rules! deps {
    () => {
        IUnknown!();
    };
}

macro_rules! impl_205 {
    () => {
        deps!();
        impl Eq for IUnknown { }
    };
}

impl_205!();