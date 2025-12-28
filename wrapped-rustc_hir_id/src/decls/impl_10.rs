macro_rules! deps {
    () => {
        HirId!();
    };
}

macro_rules! impl_10 {
    () => {
        deps!();
        impl ! PartialOrd for HirId { }
    };
}

impl_10!();