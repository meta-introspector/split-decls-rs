macro_rules! deps {
    () => {
        IntoCString!();
        Error!();
    };
}

macro_rules! impl_79 {
    () => {
        deps!();
        impl IntoCString for Vec < u8 > { fn into_c_string (self) -> Result < CString , Error > { Ok (CString :: new (self) ?) } }
    };
}

impl_79!();