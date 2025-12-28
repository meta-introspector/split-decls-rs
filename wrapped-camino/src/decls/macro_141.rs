macro_rules! deps {
    () => {
        Utf8Path!();
    };
}

macro_rules! macro_141 {
    () => {
        deps!();
        impl_cmp_std_path ! (Utf8Path , PathBuf) ;
    };
}

macro_141!()