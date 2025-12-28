macro_rules! deps {
    () => {
        Utf8Path!();
    };
}

macro_rules! macro_162 {
    () => {
        deps!();
        impl_cmp_os_str ! (Utf8Path , OsStr) ;
    };
}

macro_162!()