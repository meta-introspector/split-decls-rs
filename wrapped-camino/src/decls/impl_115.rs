macro_rules! deps {
    () => {
        Utf8PathBuf!();
    };
}

macro_rules! impl_115 {
    () => {
        deps!();
        impl PartialEq for Utf8PathBuf { # [inline] fn eq (& self , other : & Utf8PathBuf) -> bool { self . components () == other . components () } }
    };
}

impl_115!()