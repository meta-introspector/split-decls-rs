macro_rules! deps {
    () => {
        Empty!();
    };
}

macro_rules! impl_764 {
    () => {
        deps!();
        impl < T > Clone for Empty < T > { fn clone (& self) -> Self { empty () } }
    };
}

impl_764!();