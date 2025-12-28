macro_rules! deps {
    () => {
        Utf8PathBuf!();
    };
}

macro_rules! macro_137 {
    () => {
        deps!();
        impl_cmp_std_path ! (Utf8PathBuf , PathBuf) ;
    };
}

macro_137!()