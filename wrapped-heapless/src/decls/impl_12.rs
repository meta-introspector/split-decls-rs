macro_rules! deps {
    () => {
        LenType!();
        CString!();
    };
}

macro_rules! impl_12 {
    () => {
        deps!();
        impl < const N : usize , const M : usize , LenT1 : LenType , LenT2 : LenType > PartialOrd < CString < M , LenT2 > > for CString < N , LenT1 > { # [inline] fn partial_cmp (& self , rhs : & CString < M , LenT2 >) -> Option < Ordering > { self . as_c_str () . partial_cmp (rhs . as_c_str ()) } }
    };
}

impl_12!();