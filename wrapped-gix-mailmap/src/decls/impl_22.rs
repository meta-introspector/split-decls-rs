macro_rules! deps {
    () => {
        EncodedStringRef!();
        EncodedString!();
    };
}

macro_rules! impl_22 {
    () => {
        deps!();
        impl < 'a > From < EncodedStringRef < 'a > > for EncodedString { fn from (v : EncodedStringRef < 'a >) -> Self { match v { EncodedStringRef :: Utf8 (v) => EncodedString :: Utf8 (v . to_owned ()) , EncodedStringRef :: Unknown (v) => EncodedString :: Unknown (v . to_owned ()) , } } }
    };
}

impl_22!()