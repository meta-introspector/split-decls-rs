macro_rules! deps {
    () => {
        PropertyName!();
    };
}

macro_rules! impl_323 {
    () => {
        deps!();
        impl core :: cmp :: PartialEq < String > for PropertyName { # [inline] fn eq (& self , other : & String) -> bool { self . as_str () . eq (other . as_str ()) } }
    };
}

impl_323!()