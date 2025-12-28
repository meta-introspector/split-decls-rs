macro_rules! deps {
    () => {
        OldestOrdered!();
    };
}

macro_rules! impl_74 {
    () => {
        deps!();
        impl < T > Clone for OldestOrdered < '_ , T > { fn clone (& self) -> Self { Self { inner : self . inner . clone () , } } }
    };
}

impl_74!();