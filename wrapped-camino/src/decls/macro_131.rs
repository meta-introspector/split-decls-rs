macro_rules! deps {
    () => {
        Utf8Path!();
    };
}

macro_rules! macro_131 {
    () => {
        deps!();
        impl_cmp ! (Cow <'a , Utf8Path >, &'b Utf8Path) ;
    };
}

macro_131!()