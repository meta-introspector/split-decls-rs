macro_rules! deps {
    () => {
        Utf8PathBuf!();
    };
}

macro_rules! macro_166 {
    () => {
        deps!();
        impl_cmp_str ! (Utf8PathBuf , String) ;
    };
}

macro_166!();