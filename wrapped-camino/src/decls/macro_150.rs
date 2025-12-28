macro_rules! deps {
    () => {
        Utf8Path!();
    };
}

macro_rules! macro_150 {
    () => {
        deps!();
        impl_cmp_str ! (Utf8Path , str) ;
    };
}

macro_150!()