macro_rules! deps {
    () => {
        Utf8PathBuf!();
    };
}

macro_rules! macro_160 {
    () => {
        deps!();
        impl_cmp_os_str ! (Utf8PathBuf , Cow <'a , OsStr >) ;
    };
}

macro_160!()