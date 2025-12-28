macro_rules! deps {
    () => {
        PropName!();
    };
}

macro_rules! impl_312 {
    () => {
        deps!();
        impl core :: cmp :: PartialEq < PropName > for str { # [inline] fn eq (& self , other : & PropName) -> bool { self . eq (other . as_str ()) } }
    };
}

impl_312!()