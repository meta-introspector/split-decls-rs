macro_rules! deps {
    () => {
        Positioned!();
    };
}

macro_rules! impl_125 {
    () => {
        deps!();
        impl < T : Eq > Eq for Positioned < T > { }
    };
}

impl_125!();