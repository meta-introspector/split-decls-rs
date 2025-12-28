macro_rules! deps {
    () => {
        Builder!();
    };
}

macro_rules! impl_163 {
    () => {
        deps!();
        impl abi :: HasDataLayout for Builder < '_ , '_ , '_ > { fn data_layout (& self) -> & abi :: TargetDataLayout { self . cx . data_layout () } }
    };
}

impl_163!()