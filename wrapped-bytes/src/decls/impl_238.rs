macro_rules! deps {
    () => {
        BytesMut!();
    };
}

macro_rules! impl_238 {
    () => {
        deps!();
        impl PartialEq < BytesMut > for & str { fn eq (& self , other : & BytesMut) -> bool { * other == * self } }
    };
}

impl_238!();