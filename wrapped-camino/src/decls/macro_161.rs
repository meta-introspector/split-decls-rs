macro_rules! deps {
    () => {
        Utf8Path!();
    };
}

macro_rules! macro_161 {
    () => {
        deps!();
        impl_cmp_std_path ! (&'a Utf8Path , PathBuf) ;
    };
}

macro_161!()