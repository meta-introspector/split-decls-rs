macro_rules! deps {
    () => {
        Drain!();
    };
}

macro_rules! impl_922 {
    () => {
        deps!();
        impl < T > Clone for Drain < T > { fn clone (& self) -> Self { drain () } }
    };
}

impl_922!();