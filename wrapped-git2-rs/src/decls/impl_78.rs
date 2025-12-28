macro_rules! deps {
    () => {
        IntoCString!();
        Error!();
    };
}

macro_rules! impl_78 {
    () => {
        deps!();
        impl < 'a > IntoCString for & 'a [u8] { fn into_c_string (self) -> Result < CString , Error > { Ok (CString :: new (self) ?) } }
    };
}

impl_78!();