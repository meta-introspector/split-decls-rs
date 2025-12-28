macro_rules! deps {
    () => {
        IeeeFloat!();
    };
}

macro_rules! impl_30 {
    () => {
        deps!();
        impl < S > Clone for IeeeFloat < S > { fn clone (& self) -> Self { * self } }
    };
}

impl_30!()