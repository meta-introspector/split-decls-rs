macro_rules! deps {
    () => {
        Utf8PathBuf!();
    };
}

macro_rules! macro_136 {
    () => {
        deps!();
        impl_cmp_std_path ! (Utf8PathBuf , Cow <'a , Path >) ;
    };
}

macro_136!()