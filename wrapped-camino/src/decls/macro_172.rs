macro_rules! deps {
    () => {
        Utf8Path!();
    };
}

macro_rules! macro_172 {
    () => {
        deps!();
        impl_cmp_str ! (&'a Utf8Path , Cow <'b , str >) ;
    };
}

macro_172!();