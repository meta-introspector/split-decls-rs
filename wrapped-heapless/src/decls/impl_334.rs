macro_rules! deps {
    () => {
        LenType!();
        Vec!();
    };
}

macro_rules! impl_334 {
    () => {
        deps!();
        impl < T , const N : usize , LenT : LenType > Clone for Vec < T , N , LenT > where T : Clone , { fn clone (& self) -> Self { self . clone () } }
    };
}

impl_334!();