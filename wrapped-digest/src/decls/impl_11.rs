macro_rules! deps {
    () => {
        CtOutput!();
    };
}

macro_rules! impl_11 {
    () => {
        deps!();
        impl < T : OutputSizeUser > Eq for CtOutput < T > { }
    };
}

impl_11!()