macro_rules! deps {
    () => {
        Own!();
    };
}

macro_rules! impl_122 {
    () => {
        deps!();
        impl < T > Clone for Own < T > where T : ? Sized , { fn clone (& self) -> Self { * self } }
    };
}

impl_122!();