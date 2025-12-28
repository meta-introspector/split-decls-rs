macro_rules! deps {
    () => {
        Error!();
        IntoCString!();
    };
}

macro_rules! impl_74 {
    () => {
        deps!();
        impl < 'a > IntoCString for & 'a Path { fn into_c_string (self) -> Result < CString , Error > { let s : & OsStr = self . as_ref () ; s . into_c_string () } }
    };
}

impl_74!()