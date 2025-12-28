macro_rules! deps {
    () => {
        Utf8PathBuf!();
    };
}

macro_rules! macro_146 {
    () => {
        deps!();
        impl_cmp_str ! (Utf8PathBuf , str) ;
    };
}

macro_146!()