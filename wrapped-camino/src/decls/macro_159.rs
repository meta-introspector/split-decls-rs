macro_rules! deps {
    () => {
        Utf8Path!();
    };
}

macro_rules! macro_159 {
    () => {
        deps!();
        impl_cmp_std_path ! (&'a Utf8Path , Path) ;
    };
}

macro_159!()