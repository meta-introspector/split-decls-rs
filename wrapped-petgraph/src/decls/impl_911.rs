macro_rules! deps {
    () => {
        Ptr!();
    };
}

macro_rules! impl_911 {
    () => {
        deps!();
        impl < T > Clone for Ptr < '_ , T > { fn clone (& self) -> Self { * self } }
    };
}

impl_911!();