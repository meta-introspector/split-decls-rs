macro_rules! deps {
    () => {
        Utf8Path!();
    };
}

macro_rules! macro_164 {
    () => {
        deps!();
        impl_cmp_os_str ! (Utf8Path , Cow <'a , OsStr >) ;
    };
}

macro_164!()