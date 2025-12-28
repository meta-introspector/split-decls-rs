macro_rules! deps {
    () => {
        Utf8PathBuf!();
        Utf8Path!();
    };
}

macro_rules! macro_132 {
    () => {
        deps!();
        impl_cmp ! (Cow <'a , Utf8Path >, Utf8PathBuf) ;
    };
}

macro_132!()