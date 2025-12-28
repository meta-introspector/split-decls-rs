macro_rules! deps {
    () => {
        ByteString!();
    };
}

macro_rules! impl_1088 {
    () => {
        deps!();
        impl < 'a > From < Vec < u8 > > for ByteString < 'a > { fn from (bytes : Vec < u8 >) -> Self { ByteString (Cow :: Owned (bytes)) } }
    };
}

impl_1088!();