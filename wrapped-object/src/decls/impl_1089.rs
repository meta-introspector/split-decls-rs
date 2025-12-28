macro_rules! deps {
    () => {
        ByteString!();
    };
}

macro_rules! impl_1089 {
    () => {
        deps!();
        impl < 'a > From < & 'a str > for ByteString < 'a > { fn from (s : & 'a str) -> Self { ByteString (Cow :: Borrowed (s . as_bytes ())) } }
    };
}

impl_1089!()