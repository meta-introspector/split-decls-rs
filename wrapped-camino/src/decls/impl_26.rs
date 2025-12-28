macro_rules! deps {
    () => {
        Utf8PathBuf!();
        Utf8Path!();
    };
}

macro_rules! impl_26 {
    () => {
        deps!();
        impl < P : AsRef < Utf8Path > > Extend < P > for Utf8PathBuf { fn extend < I : IntoIterator < Item = P > > (& mut self , iter : I) { for path in iter { self . push (path) ; } } }
    };
}

impl_26!();