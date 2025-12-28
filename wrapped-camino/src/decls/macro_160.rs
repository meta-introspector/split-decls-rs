macro_rules! deps {
    () => {
        Utf8Path!();
    };
}

macro_rules! macro_160 {
    () => {
        deps!();
        impl_cmp_std_path ! (&'a Utf8Path , Cow <'b , Path >) ;
    };
}

macro_160!()