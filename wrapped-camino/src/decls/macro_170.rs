macro_rules! deps {
    () => {
        Utf8Path!();
    };
}

macro_rules! macro_170 {
    () => {
        deps!();
        impl_cmp_str ! (Utf8Path , String) ;
    };
}

macro_170!()