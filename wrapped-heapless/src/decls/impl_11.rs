macro_rules! deps {
    () => {
        CString!();
        LenType!();
    };
}

macro_rules! impl_11 {
    () => {
        deps!();
        impl < const N : usize , LenT : LenType > Eq for CString < N , LenT > { }
    };
}

impl_11!()