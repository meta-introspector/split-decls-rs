macro_rules! deps {
    () => {
        BytesMut!();
        Bytes!();
    };
}

macro_rules! impl_240 {
    () => {
        deps!();
        impl PartialEq < BytesMut > for Bytes { fn eq (& self , other : & BytesMut) -> bool { other [..] == self [..] } }
    };
}

impl_240!()