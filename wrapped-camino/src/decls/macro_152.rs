macro_rules! deps {
    () => {
        Utf8Path!();
    };
}

macro_rules! macro_152 {
    () => {
        deps!();
        impl_cmp_str ! (Utf8Path , Cow <'a , str >) ;
    };
}

macro_152!()