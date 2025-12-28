macro_rules! deps {
    () => {
        PropertyName!();
    };
}

macro_rules! impl_324 {
    () => {
        deps!();
        impl core :: cmp :: PartialEq < PropertyName > for CString { # [inline] fn eq (& self , other : & PropertyName) -> bool { self . as_c_str () . eq (other . as_c_str ()) } }
    };
}

impl_324!()