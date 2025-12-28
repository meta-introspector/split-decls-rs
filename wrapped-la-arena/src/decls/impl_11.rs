macro_rules! deps {
    () => {
        Idx!();
    };
}

macro_rules! impl_11 {
    () => {
        deps!();
        impl < T > Copy for Idx < T > { }
    };
}

impl_11!()