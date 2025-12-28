macro_rules! deps {
    () => {
        GenThenTime!();
    };
}

macro_rules! impl_21 {
    () => {
        deps!();
        impl PartialEq < Self > for GenThenTime { fn eq (& self , other : & Self) -> bool { self . cmp (other) . is_eq () } }
    };
}

impl_21!();