macro_rules! deps {
    () => {
        BytesMut!();
    };
}

macro_rules! impl_190 {
    () => {
        deps!();
        impl < 'a > From < & 'a [u8] > for BytesMut { fn from (src : & 'a [u8]) -> BytesMut { BytesMut :: from_vec (src . to_vec ()) } }
    };
}

impl_190!()