macro_rules! deps {
    () => {
        CtOutput!();
    };
}

macro_rules! impl_64 {
    () => {
        deps!();
        impl < T : OutputSizeUser > Eq for CtOutput < T > { }
    };
}

impl_64!()