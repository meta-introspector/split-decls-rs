macro_rules! deps {
    () => {
        EncodedStringRef!();
    };
}

macro_rules! impl_21 {
    () => {
        deps!();
        impl < 'a > From < & 'a BStr > for EncodedStringRef < 'a > { fn from (v : & 'a BStr) -> Self { match v . to_str () { Ok (v) => EncodedStringRef :: Utf8 (v) , Err (_) => EncodedStringRef :: Unknown (v) , } } }
    };
}

impl_21!()