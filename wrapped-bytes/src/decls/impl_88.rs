macro_rules! deps {
    () => {
        Bytes!();
    };
}

macro_rules! impl_88 {
    () => {
        deps!();
        impl PartialEq < str > for Bytes { fn eq (& self , other : & str) -> bool { self . as_slice () == other . as_bytes () } }
    };
}

impl_88!()