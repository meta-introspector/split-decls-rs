macro_rules! deps {
    () => {
        Utf8PathBuf!();
    };
}

macro_rules! macro_161 {
    () => {
        deps!();
        impl_cmp_os_str ! (Utf8PathBuf , OsString) ;
    };
}

macro_161!()