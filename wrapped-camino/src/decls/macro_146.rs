macro_rules! deps {
    () => {
        Utf8PathBuf!();
        Utf8Path!();
    };
}

macro_rules! macro_146 {
    () => {
        deps!();
        impl_cmp ! (Utf8PathBuf , &'a Utf8Path) ;
    };
}

macro_146!();