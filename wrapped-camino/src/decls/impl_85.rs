macro_rules! deps {
    () => {
        Utf8Path!();
    };
}

macro_rules! impl_85 {
    () => {
        deps!();
        impl < 'a > From < & 'a Utf8Path > for Cow < 'a , Path > { fn from (path : & 'a Utf8Path) -> Cow < 'a , Path > { Cow :: Borrowed (path . as_ref ()) } }
    };
}

impl_85!()