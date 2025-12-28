macro_rules! deps {
    () => {
        LenType!();
        String!();
    };
}

macro_rules! impl_238 {
    () => {
        deps!();
        impl < LenT : LenType , const N : usize > Clone for String < N , LenT > { fn clone (& self) -> Self { Self { vec : self . vec . clone () , } } }
    };
}

impl_238!()