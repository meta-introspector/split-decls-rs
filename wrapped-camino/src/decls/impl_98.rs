macro_rules! deps {
    () => {
        Utf8PathBuf!();
    };
}

macro_rules! impl_98 {
    () => {
        deps!();
        impl < 'a > From < Utf8PathBuf > for Cow < 'a , Path > { fn from (path : Utf8PathBuf) -> Cow < 'a , Path > { PathBuf :: from (path) . into () } }
    };
}

impl_98!()