macro_rules! deps {
    () => {
        Utf8PathBuf!();
    };
}

macro_rules! macro_151 {
    () => {
        deps!();
        impl_cmp_std_path ! (Utf8PathBuf , Path) ;
    };
}

macro_151!();