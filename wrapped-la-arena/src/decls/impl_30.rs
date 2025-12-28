macro_rules! deps {
    () => {
        Idx!();
    };
}

macro_rules! impl_30 {
    () => {
        deps!();
        impl < T > Copy for Idx < T > { }
    };
}

impl_30!();