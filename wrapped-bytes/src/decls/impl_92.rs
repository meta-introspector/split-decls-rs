macro_rules! deps {
    () => {
        Bytes!();
    };
}

macro_rules! impl_92 {
    () => {
        deps!();
        impl PartialEq < Vec < u8 > > for Bytes { fn eq (& self , other : & Vec < u8 >) -> bool { * self == other [..] } }
    };
}

impl_92!();