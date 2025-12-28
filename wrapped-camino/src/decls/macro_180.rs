macro_rules! deps {
    () => {
        Utf8Path!();
    };
}

macro_rules! macro_180 {
    () => {
        deps!();
        impl_cmp_os_str ! (Utf8Path , &'a OsStr) ;
    };
}

macro_180!();