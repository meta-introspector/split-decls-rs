macro_rules! deps {
    () => {
        Utf8Path!();
    };
}

macro_rules! macro_166 {
    () => {
        deps!();
        impl_cmp_os_str ! (&'a Utf8Path , OsStr) ;
    };
}

macro_166!()