macro_rules! deps {
    () => {
        Utf8PathBuf!();
    };
}

macro_rules! impl_133 {
    () => {
        deps!();
        impl Eq for Utf8PathBuf { }
    };
}

impl_133!();