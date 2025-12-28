macro_rules! deps {
    () => {
        CString!();
        LenType!();
    };
}

macro_rules! impl_10 {
    () => {
        deps!();
        impl < const N : usize , const M : usize , LenT1 : LenType , LenT2 : LenType > PartialEq < CString < M , LenT2 > > for CString < N , LenT1 > { # [inline] fn eq (& self , rhs : & CString < M , LenT2 >) -> bool { self . as_c_str () == rhs . as_c_str () } }
    };
}

impl_10!()