macro_rules! deps {
    () => {
        RootDatabase!();
    };
}

macro_rules! impl_247 {
    () => {
        deps!();
        impl std :: panic :: RefUnwindSafe for RootDatabase { }
    };
}

impl_247!();