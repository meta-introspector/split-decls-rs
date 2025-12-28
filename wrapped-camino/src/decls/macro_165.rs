macro_rules! deps {
    () => {
        Utf8PathBuf!();
    };
}

macro_rules! macro_165 {
    () => {
        deps!();
        impl_cmp_str ! (Utf8PathBuf , Cow <'a , str >) ;
    };
}

macro_165!()