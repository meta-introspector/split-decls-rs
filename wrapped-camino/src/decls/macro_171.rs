macro_rules! deps {
    () => {
        Utf8Path!();
    };
}

macro_rules! macro_171 {
    () => {
        deps!();
        impl_cmp_str ! (&'a Utf8Path , str) ;
    };
}

macro_171!()