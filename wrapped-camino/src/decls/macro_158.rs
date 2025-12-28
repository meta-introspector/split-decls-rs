macro_rules! deps {
    () => {
        Utf8PathBuf!();
    };
}

macro_rules! macro_158 {
    () => {
        deps!();
        impl_cmp_os_str ! (Utf8PathBuf , OsStr) ;
    };
}

macro_158!()