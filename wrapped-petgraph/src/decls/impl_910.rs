macro_rules! deps {
    () => {
        Ptr!();
    };
}

macro_rules! impl_910 {
    () => {
        deps!();
        impl < T > Copy for Ptr < '_ , T > { }
    };
}

impl_910!();