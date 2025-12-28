macro_rules! deps {
    () => {
        BytesMut!();
    };
}

macro_rules! impl_202 {
    () => {
        deps!();
        impl Clone for BytesMut { fn clone (& self) -> BytesMut { BytesMut :: from (& self [..]) } }
    };
}

impl_202!()