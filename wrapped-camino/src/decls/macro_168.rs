macro_rules! deps {
    () => {
        Utf8Path!();
    };
}

macro_rules! macro_168 {
    () => {
        deps!();
        impl_cmp_str ! (Utf8Path , &'a str) ;
    };
}

macro_168!()