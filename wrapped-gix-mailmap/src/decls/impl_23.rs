macro_rules! deps {
    () => {
        EncodedString!();
    };
}

macro_rules! impl_23 {
    () => {
        deps!();
        impl < 'a > From < & 'a BStr > for EncodedString { fn from (v : & 'a BStr) -> Self { match v . to_str () { Ok (v) => EncodedString :: Utf8 (v . to_owned ()) , Err (_) => EncodedString :: Unknown (v . to_owned ()) , } } }
    };
}

impl_23!()