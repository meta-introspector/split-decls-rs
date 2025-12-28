macro_rules! deps {
    () => {
        LenType!();
        CString!();
    };
}

macro_rules! impl_6 {
    () => {
        deps!();
        impl < const N : usize , LenT : LenType > AsRef < CStr > for CString < N , LenT > { # [inline] fn as_ref (& self) -> & CStr { self . as_c_str () } }
    };
}

impl_6!()