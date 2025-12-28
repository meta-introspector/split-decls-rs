macro_rules! deps {
    () => {
        Utf8PathBuf!();
    };
}

macro_rules! impl_136 {
    () => {
        deps!();
        impl Ord for Utf8PathBuf { fn cmp (& self , other : & Utf8PathBuf) -> Ordering { self . components () . cmp (other . components ()) } }
    };
}

impl_136!()