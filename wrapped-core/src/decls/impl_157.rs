macro_rules! deps {
    () => {
        InterfaceRef!();
    };
}

macro_rules! impl_157 {
    () => {
        deps!();
        impl < I > Clone for InterfaceRef < '_ , I > { fn clone (& self) -> Self { * self } }
    };
}

impl_157!();