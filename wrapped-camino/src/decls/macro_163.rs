macro_rules! deps {
    () => {
        Utf8Path!();
    };
}

macro_rules! macro_163 {
    () => {
        deps!();
        impl_cmp_os_str ! (Utf8Path , &'a OsStr) ;
    };
}

macro_163!()