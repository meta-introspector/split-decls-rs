macro_rules! deps {
    () => {
        Utf8Path!();
    };
}

macro_rules! macro_173 {
    () => {
        deps!();
        impl_cmp_str ! (&'a Utf8Path , String) ;
    };
}

macro_173!()