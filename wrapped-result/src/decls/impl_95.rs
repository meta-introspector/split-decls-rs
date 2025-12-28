macro_rules! deps {
    () => {
        BOOL!();
    };
}

macro_rules! impl_95 {
    () => {
        deps!();
        impl PartialEq < BOOL > for bool { fn eq (& self , other : & BOOL) -> bool { * self == other . as_bool () } }
    };
}

impl_95!()