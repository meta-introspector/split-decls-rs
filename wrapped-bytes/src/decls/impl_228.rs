macro_rules! deps {
    () => {
        BytesMut!();
    };
}

macro_rules! impl_228 {
    () => {
        deps!();
        impl PartialEq < BytesMut > for Vec < u8 > { fn eq (& self , other : & BytesMut) -> bool { * other == * self } }
    };
}

impl_228!()