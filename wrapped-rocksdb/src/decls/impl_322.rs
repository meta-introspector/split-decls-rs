macro_rules! deps {
    () => {
        PropertyName!();
    };
}

macro_rules! impl_322 {
    () => {
        deps!();
        impl core :: cmp :: PartialEq < CString > for PropertyName { # [inline] fn eq (& self , other : & CString) -> bool { self . as_c_str () . eq (other . as_c_str ()) } }
    };
}

impl_322!();