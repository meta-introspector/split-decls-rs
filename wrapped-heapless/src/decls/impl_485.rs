macro_rules! deps {
    () => {
        CString!();
        LenType!();
        ExtendError!();
    };
}

macro_rules! impl_485 {
    () => {
        deps!();
        impl < const N : usize , LenT : LenType > uWrite for CString < N , LenT > { type Error = c_string :: ExtendError ; # [inline] fn write_str (& mut self , s : & str) -> Result < () , Self :: Error > { self . extend_from_bytes (s . as_bytes ()) } }
    };
}

impl_485!();