macro_rules! deps {
    () => {
        Bytes!();
    };
}

macro_rules! impl_1075 {
    () => {
        deps!();
        impl < 'a > From < & 'a [u8] > for Bytes < 'a > { fn from (bytes : & 'a [u8]) -> Self { Bytes (Cow :: Borrowed (bytes)) } }
    };
}

impl_1075!();