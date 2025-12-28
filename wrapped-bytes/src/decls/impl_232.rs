macro_rules! deps {
    () => {
        BytesMut!();
    };
}

macro_rules! impl_232 {
    () => {
        deps!();
        impl PartialEq < BytesMut > for String { fn eq (& self , other : & BytesMut) -> bool { * other == * self } }
    };
}

impl_232!();