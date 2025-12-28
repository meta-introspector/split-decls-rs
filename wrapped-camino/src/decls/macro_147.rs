macro_rules! deps {
    () => {
        Utf8Path!();
    };
}

macro_rules! macro_147 {
    () => {
        deps!();
        impl_cmp ! (Cow <'a , Utf8Path >, Utf8Path) ;
    };
}

macro_147!()