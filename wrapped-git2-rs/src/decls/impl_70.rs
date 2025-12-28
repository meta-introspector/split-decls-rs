macro_rules! deps {
    () => {
        Error!();
        IntoCString!();
    };
}

macro_rules! impl_70 {
    () => {
        deps!();
        impl < 'a , T : IntoCString + Clone > IntoCString for & 'a T { fn into_c_string (self) -> Result < CString , Error > { self . clone () . into_c_string () } }
    };
}

impl_70!()