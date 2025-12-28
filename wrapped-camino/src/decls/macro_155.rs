macro_rules! deps {
    () => {
        Utf8Path!();
    };
}

macro_rules! macro_155 {
    () => {
        deps!();
        impl_cmp_std_path ! (Utf8Path , Path) ;
    };
}

macro_155!();