macro_rules! deps {
    () => {
        CString!();
        LenType!();
    };
}

macro_rules! impl_14 {
    () => {
        deps!();
        impl < const N : usize , LenT : LenType > fmt :: Debug for CString < N , LenT > { fn fmt (& self , f : & mut fmt :: Formatter < '_ >) -> fmt :: Result { self . as_c_str () . fmt (f) } }
    };
}

impl_14!()