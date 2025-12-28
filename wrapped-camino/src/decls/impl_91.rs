macro_rules! deps {
    () => {
        Utf8PathBuf!();
        Utf8Path!();
    };
}

macro_rules! impl_91 {
    () => {
        deps!();
        impl < 'a > From < Utf8PathBuf > for Cow < 'a , Utf8Path > { fn from (path : Utf8PathBuf) -> Cow < 'a , Utf8Path > { Cow :: Owned (path) } }
    };
}

impl_91!()