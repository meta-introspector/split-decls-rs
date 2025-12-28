macro_rules! deps {
    () => {
        Idx!();
    };
}

macro_rules! impl_32 {
    () => {
        deps!();
        impl < T > Eq for Idx < T > { }
    };
}

impl_32!();