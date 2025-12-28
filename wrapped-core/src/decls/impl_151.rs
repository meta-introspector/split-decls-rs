macro_rules! deps {
    () => {
        IInspectable!();
        RuntimeName!();
    };
}

macro_rules! impl_151 {
    () => {
        deps!();
        impl RuntimeName for IInspectable { }
    };
}

impl_151!();