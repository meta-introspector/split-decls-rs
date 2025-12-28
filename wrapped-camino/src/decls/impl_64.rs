macro_rules! deps {
    () => {
        Utf8Path!();
    };
}

macro_rules! impl_64 {
    () => {
        deps!();
        impl < 'a > From < & 'a Utf8Path > for Cow < 'a , Utf8Path > { fn from (path : & 'a Utf8Path) -> Cow < 'a , Utf8Path > { Cow :: Borrowed (path) } }
    };
}

impl_64!()