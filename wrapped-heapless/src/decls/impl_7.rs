macro_rules! deps {
    () => {
        CString!();
        LenType!();
    };
}

macro_rules! impl_7 {
    () => {
        deps!();
        impl < const N : usize , LenT : LenType > Borrow < CStr > for CString < N , LenT > { # [inline] fn borrow (& self) -> & CStr { self . as_c_str () } }
    };
}

impl_7!();