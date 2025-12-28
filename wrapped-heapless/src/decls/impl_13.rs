macro_rules! deps {
    () => {
        CString!();
        LenType!();
    };
}

macro_rules! impl_13 {
    () => {
        deps!();
        impl < const N : usize , LenT : LenType > Ord for CString < N , LenT > { # [inline] fn cmp (& self , rhs : & Self) -> Ordering { self . as_c_str () . cmp (rhs . as_c_str ()) } }
    };
}

impl_13!();