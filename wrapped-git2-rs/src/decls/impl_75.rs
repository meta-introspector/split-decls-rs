macro_rules! deps {
    () => {
        Error!();
        IntoCString!();
    };
}

macro_rules! impl_75 {
    () => {
        deps!();
        impl IntoCString for PathBuf { fn into_c_string (self) -> Result < CString , Error > { let s : OsString = self . into () ; s . into_c_string () } }
    };
}

impl_75!()