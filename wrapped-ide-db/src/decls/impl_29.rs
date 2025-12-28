macro_rules! deps {
    () => {
        RootDatabase!();
    };
}

macro_rules! impl_29 {
    () => {
        deps!();
        impl std :: panic :: RefUnwindSafe for RootDatabase { }
    };
}

impl_29!()