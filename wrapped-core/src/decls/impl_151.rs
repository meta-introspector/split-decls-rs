macro_rules! deps {
    () => {
        RuntimeName!();
        IInspectable!();
    };
}

macro_rules! impl_151 {
    () => {
        deps!();
        impl RuntimeName for IInspectable { }
    };
}

impl_151!()