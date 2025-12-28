macro_rules! deps {
    () => {
        Utf8PathBuf!();
    };
}

macro_rules! macro_149 {
    () => {
        deps!();
        impl_cmp_str ! (Utf8PathBuf , String) ;
    };
}

macro_149!()