macro_rules! deps {
    () => {
        IntoCString!();
        Error!();
    };
}

macro_rules! impl_71 {
    () => {
        deps!();
        impl < 'a > IntoCString for & 'a str { fn into_c_string (self) -> Result < CString , Error > { Ok (CString :: new (self) ?) } }
    };
}

impl_71!()