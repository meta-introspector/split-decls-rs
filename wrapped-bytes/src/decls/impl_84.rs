macro_rules! deps {
    () => {
        Bytes!();
    };
}

macro_rules! impl_84 {
    () => {
        deps!();
        impl PartialEq < [u8] > for Bytes { fn eq (& self , other : & [u8]) -> bool { self . as_slice () == other } }
    };
}

impl_84!();