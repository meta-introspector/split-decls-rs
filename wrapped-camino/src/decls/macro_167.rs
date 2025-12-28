macro_rules! deps {
    () => {
        Utf8Path!();
    };
}

macro_rules! macro_167 {
    () => {
        deps!();
        impl_cmp_str ! (Utf8Path , str) ;
    };
}

macro_167!()