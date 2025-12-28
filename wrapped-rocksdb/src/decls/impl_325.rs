macro_rules! deps {
    () => {
        PropertyName!();
    };
}

macro_rules! impl_325 {
    () => {
        deps!();
        impl core :: cmp :: PartialEq < PropertyName > for String { # [inline] fn eq (& self , other : & PropertyName) -> bool { self . as_str () . eq (other . as_str ()) } }
    };
}

impl_325!();