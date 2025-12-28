macro_rules! deps {
    () => {
        Utf8Path!();
    };
}

macro_rules! macro_168 {
    () => {
        deps!();
        impl_cmp_os_str ! (&'a Utf8Path , OsString) ;
    };
}

macro_168!()