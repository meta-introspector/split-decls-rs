macro_rules! deps {
    () => {
        Hasher!();
    };
}

macro_rules! impl_125 {
    () => {
        deps!();
        impl digest :: HashMarker for Hasher { }
    };
}

impl_125!();