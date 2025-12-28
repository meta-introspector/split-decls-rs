macro_rules! deps {
    () => {
        Pairs!();
    };
}

macro_rules! impl_57 {
    () => {
        deps!();
        impl < R : Eq > Eq for Pairs < '_ , R > { }
    };
}

impl_57!();