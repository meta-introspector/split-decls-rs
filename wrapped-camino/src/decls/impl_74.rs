macro_rules! deps {
    () => {
        Utf8Path!();
        Utf8PathBuf!();
    };
}

macro_rules! impl_74 {
    () => {
        deps!();
        impl < 'a > From < Utf8PathBuf > for Cow < 'a , Utf8Path > { fn from (path : Utf8PathBuf) -> Cow < 'a , Utf8Path > { Cow :: Owned (path) } }
    };
}

impl_74!()