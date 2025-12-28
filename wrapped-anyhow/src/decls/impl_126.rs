macro_rules! deps {
    () => {
        Ref!();
    };
}

macro_rules! impl_126 {
    () => {
        deps!();
        impl < 'a , T > Clone for Ref < 'a , T > where T : ? Sized , { fn clone (& self) -> Self { * self } }
    };
}

impl_126!();