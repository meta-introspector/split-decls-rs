macro_rules! deps {
    () => {
        Utf8Path!();
        Utf8PathBuf!();
    };
}

macro_rules! macro_149 {
    () => {
        deps!();
        impl_cmp ! (Cow <'a , Utf8Path >, Utf8PathBuf) ;
    };
}

macro_149!()