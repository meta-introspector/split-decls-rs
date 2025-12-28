macro_rules! deps {
    () => {
        ByteString!();
    };
}

macro_rules! impl_1087 {
    () => {
        deps!();
        impl < 'a > From < & 'a [u8] > for ByteString < 'a > { fn from (bytes : & 'a [u8]) -> Self { ByteString (Cow :: Borrowed (bytes)) } }
    };
}

impl_1087!();