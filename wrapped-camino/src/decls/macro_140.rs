macro_rules! deps {
    () => {
        Utf8Path!();
    };
}

macro_rules! macro_140 {
    () => {
        deps!();
        impl_cmp_std_path ! (Utf8Path , Cow <'a , Path >) ;
    };
}

macro_140!()