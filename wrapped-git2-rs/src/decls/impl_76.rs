macro_rules! deps {
    () => {
        Error!();
        IntoCString!();
    };
}

macro_rules! impl_76 {
    () => {
        deps!();
        impl < 'a > IntoCString for & 'a OsStr { fn into_c_string (self) -> Result < CString , Error > { self . to_os_string () . into_c_string () } }
    };
}

impl_76!()