macro_rules! deps {
    () => {
        Shared!();
        Pointable!();
    };
}

macro_rules! impl_50 {
    () => {
        deps!();
        impl < T : ? Sized + Pointable > Clone for Shared < '_ , T > { fn clone (& self) -> Self { * self } }
    };
}

impl_50!()