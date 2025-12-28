macro_rules! deps {
    () => {
        BOOL!();
    };
}

macro_rules! impl_94 {
    () => {
        deps!();
        impl PartialEq < bool > for BOOL { fn eq (& self , other : & bool) -> bool { self . as_bool () == * other } }
    };
}

impl_94!()