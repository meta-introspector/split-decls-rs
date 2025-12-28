macro_rules! deps {
    () => {
        Utf8Path!();
    };
}

macro_rules! macro_169 {
    () => {
        deps!();
        impl_cmp_str ! (Utf8Path , Cow <'a , str >) ;
    };
}

macro_169!()