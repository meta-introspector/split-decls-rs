macro_rules! deps {
    () => {
        Utf8PathBuf!();
        Utf8Path!();
    };
}

macro_rules! impl_88 {
    () => {
        deps!();
        impl < 'a > From < Cow < 'a , Utf8Path > > for Utf8PathBuf { fn from (path : Cow < 'a , Utf8Path >) -> Utf8PathBuf { path . into_owned () } }
    };
}

impl_88!()