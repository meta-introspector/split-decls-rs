macro_rules! deps {
    () => {
        Ref!();
    };
}

macro_rules! impl_92 {
    () => {
        deps!();
        impl < T > Copy for Ref < T > { }
    };
}

impl_92!();