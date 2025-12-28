macro_rules! deps {
    () => {
        Utf8Path!();
    };
}

macro_rules! macro_153 {
    () => {
        deps!();
        impl_cmp_str ! (Utf8Path , String) ;
    };
}

macro_153!()