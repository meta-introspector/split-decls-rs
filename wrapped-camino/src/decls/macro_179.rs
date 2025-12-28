macro_rules! deps {
    () => {
        Utf8Path!();
    };
}

macro_rules! macro_179 {
    () => {
        deps!();
        impl_cmp_os_str ! (Utf8Path , OsStr) ;
    };
}

macro_179!()