macro_rules! deps {
    () => {
        IntoCString!();
        Error!();
    };
}

macro_rules! impl_73 {
    () => {
        deps!();
        impl IntoCString for CString { fn into_c_string (self) -> Result < CString , Error > { Ok (self) } }
    };
}

impl_73!()