macro_rules! deps {
    () => {
        Ptr!();
    };
}

macro_rules! impl_917 {
    () => {
        deps!();
        impl < T > Eq for Ptr < '_ , T > { }
    };
}

impl_917!();