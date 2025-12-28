macro_rules! deps {
    () => {
        IndexStr!();
    };
}

macro_rules! impl_326 {
    () => {
        deps!();
        impl < 'a > From < & 'a [u8] > for IndexStr < 'a > { fn from (s : & 'a [u8]) -> IndexStr < 'a > { IndexStr :: new (s) } }
    };
}

impl_326!();