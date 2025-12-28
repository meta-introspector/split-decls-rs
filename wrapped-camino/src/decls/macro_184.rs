macro_rules! deps {
    () => {
        Utf8Path!();
    };
}

macro_rules! macro_184 {
    () => {
        deps!();
        impl_cmp_os_str ! (&'a Utf8Path , Cow <'b , OsStr >) ;
    };
}

macro_184!()