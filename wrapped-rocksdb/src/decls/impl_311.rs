macro_rules! deps {
    () => {
        PropName!();
    };
}

macro_rules! impl_311 {
    () => {
        deps!();
        impl core :: cmp :: PartialEq < PropName > for CStr { # [inline] fn eq (& self , other : & PropName) -> bool { self . eq (other . as_c_str ()) } }
    };
}

impl_311!()