macro_rules! deps {
    () => {
        Bytes!();
    };
}

macro_rules! impl_1076 {
    () => {
        deps!();
        impl < 'a > From < Vec < u8 > > for Bytes < 'a > { fn from (bytes : Vec < u8 >) -> Self { Bytes (Cow :: Owned (bytes)) } }
    };
}

impl_1076!();