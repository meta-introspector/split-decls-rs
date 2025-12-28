macro_rules! deps {
    () => {
        Utf8PathBuf!();
    };
}

macro_rules! macro_159 {
    () => {
        deps!();
        impl_cmp_os_str ! (Utf8PathBuf , &'a OsStr) ;
    };
}

macro_159!()