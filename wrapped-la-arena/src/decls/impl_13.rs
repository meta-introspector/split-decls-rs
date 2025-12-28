macro_rules! deps {
    () => {
        Idx!();
    };
}

macro_rules! impl_13 {
    () => {
        deps!();
        impl < T > Eq for Idx < T > { }
    };
}

impl_13!()