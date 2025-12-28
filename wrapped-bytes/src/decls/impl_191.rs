macro_rules! deps {
    () => {
        BytesMut!();
    };
}

macro_rules! impl_191 {
    () => {
        deps!();
        impl < 'a > From < & 'a str > for BytesMut { fn from (src : & 'a str) -> BytesMut { BytesMut :: from (src . as_bytes ()) } }
    };
}

impl_191!();