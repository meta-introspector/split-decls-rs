macro_rules! deps {
    () => {
        Utf8Path!();
        Utf8PathBuf!();
    };
}

macro_rules! impl_131 {
    () => {
        deps!();
        impl < P : AsRef < Utf8Path > > std :: iter :: FromIterator < P > for Utf8PathBuf { fn from_iter < I : IntoIterator < Item = P > > (iter : I) -> Utf8PathBuf { let mut buf = Utf8PathBuf :: new () ; buf . extend (iter) ; buf } }
    };
}

impl_131!();