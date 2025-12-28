macro_rules! deps {
    () => {
        BytesMut!();
        Bytes!();
    };
}

macro_rules! impl_192 {
    () => {
        deps!();
        impl From < BytesMut > for Bytes { fn from (src : BytesMut) -> Bytes { src . freeze () } }
    };
}

impl_192!()