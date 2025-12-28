macro_rules! deps {
    () => {
        Utf8Path!();
        Utf8PathBuf!();
    };
}

macro_rules! macro_128 {
    () => {
        deps!();
        impl_cmp ! (Utf8PathBuf , Utf8Path) ;
    };
}

macro_128!()