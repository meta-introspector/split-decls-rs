macro_rules! deps {
    () => {
        Utf8PathBuf!();
    };
}

macro_rules! macro_178 {
    () => {
        deps!();
        impl_cmp_os_str ! (Utf8PathBuf , OsString) ;
    };
}

macro_178!();