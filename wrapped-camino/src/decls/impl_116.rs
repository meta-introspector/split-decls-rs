macro_rules! deps {
    () => {
        Utf8PathBuf!();
    };
}

macro_rules! impl_116 {
    () => {
        deps!();
        impl Eq for Utf8PathBuf { }
    };
}

impl_116!()