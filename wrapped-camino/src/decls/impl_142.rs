macro_rules! deps {
    () => {
        Iter!();
        Utf8PathBuf!();
    };
}

macro_rules! impl_142 {
    () => {
        deps!();
        impl < 'a > IntoIterator for & 'a Utf8PathBuf { type Item = & 'a str ; type IntoIter = Iter < 'a > ; # [inline] fn into_iter (self) -> Iter < 'a > { self . iter () } }
    };
}

impl_142!();