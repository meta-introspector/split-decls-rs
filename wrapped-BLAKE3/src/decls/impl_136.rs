macro_rules! deps {
    () => {
        Hasher!();
    };
}

macro_rules! impl_136 {
    () => {
        deps!();
        impl digest :: MacMarker for Hasher { }
    };
}

impl_136!()