macro_rules! deps {
    () => {
        Utf8Path!();
    };
}

macro_rules! macro_156 {
    () => {
        deps!();
        impl_cmp_std_path ! (Utf8Path , &'a Path) ;
    };
}

macro_156!()