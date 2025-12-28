macro_rules! deps {
    () => {
        BytesMut!();
    };
}

macro_rules! impl_220 {
    () => {
        deps!();
        impl PartialEq < BytesMut > for [u8] { fn eq (& self , other : & BytesMut) -> bool { * other == * self } }
    };
}

impl_220!()