macro_rules! deps {
    () => {
        Utf8PathBuf!();
    };
}

macro_rules! macro_135 {
    () => {
        deps!();
        impl_cmp_std_path ! (Utf8PathBuf , &'a Path) ;
    };
}

macro_135!()