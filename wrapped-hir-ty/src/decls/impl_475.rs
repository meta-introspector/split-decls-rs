macro_rules! deps {
    () => {
        HirWrite!();
    };
}

macro_rules! impl_475 {
    () => {
        deps!();
        impl HirWrite for String { }
    };
}

impl_475!()