macro_rules! deps {
    () => {
        Utf8Path!();
    };
}

macro_rules! macro_183 {
    () => {
        deps!();
        impl_cmp_os_str ! (&'a Utf8Path , OsStr) ;
    };
}

macro_183!();