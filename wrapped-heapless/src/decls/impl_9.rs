macro_rules! deps {
    () => {
        CString!();
        LenType!();
    };
}

macro_rules! impl_9 {
    () => {
        deps!();
        impl < const N : usize , LenT : LenType > Deref for CString < N , LenT > { type Target = CStr ; # [inline] fn deref (& self) -> & Self :: Target { self . as_c_str () } }
    };
}

impl_9!();