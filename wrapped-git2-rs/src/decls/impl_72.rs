macro_rules! deps {
    () => {
        Error!();
        IntoCString!();
    };
}

macro_rules! impl_72 {
    () => {
        deps!();
        impl IntoCString for String { fn into_c_string (self) -> Result < CString , Error > { Ok (CString :: new (self . into_bytes ()) ?) } }
    };
}

impl_72!();