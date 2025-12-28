macro_rules! deps {
    () => {
        Utf8PathBuf!();
    };
}

macro_rules! macro_148 {
    () => {
        deps!();
        impl_cmp_str ! (Utf8PathBuf , Cow <'a , str >) ;
    };
}

macro_148!()