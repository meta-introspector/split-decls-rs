macro_rules! deps {
    () => {
        Utf8PathBuf!();
    };
}

macro_rules! macro_147 {
    () => {
        deps!();
        impl_cmp_str ! (Utf8PathBuf , &'a str) ;
    };
}

macro_147!()