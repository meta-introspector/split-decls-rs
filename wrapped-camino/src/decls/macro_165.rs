macro_rules! deps {
    () => {
        Utf8Path!();
    };
}

macro_rules! macro_165 {
    () => {
        deps!();
        impl_cmp_os_str ! (Utf8Path , OsString) ;
    };
}

macro_165!()