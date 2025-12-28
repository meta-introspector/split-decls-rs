macro_rules! deps {
    () => {
        BytesMut!();
    };
}

macro_rules! impl_226 {
    () => {
        deps!();
        impl PartialEq < Vec < u8 > > for BytesMut { fn eq (& self , other : & Vec < u8 >) -> bool { * self == other [..] } }
    };
}

impl_226!();