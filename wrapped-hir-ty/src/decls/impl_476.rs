macro_rules! deps {
    () => {
        HirWrite!();
    };
}

macro_rules! impl_476 {
    () => {
        deps!();
        impl HirWrite for fmt :: Formatter < '_ > { }
    };
}

impl_476!();