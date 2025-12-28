macro_rules! deps {
    () => {
        Bytes!();
        BytesMut!();
    };
}

macro_rules! impl_241 {
    () => {
        deps!();
        impl PartialEq < Bytes > for BytesMut { fn eq (& self , other : & Bytes) -> bool { other [..] == self [..] } }
    };
}

impl_241!();