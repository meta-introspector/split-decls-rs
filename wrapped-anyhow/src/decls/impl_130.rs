macro_rules! deps {
    () => {
        Mut!();
    };
}

macro_rules! impl_130 {
    () => {
        deps!();
        impl < 'a , T > Clone for Mut < 'a , T > where T : ? Sized , { fn clone (& self) -> Self { * self } }
    };
}

impl_130!();