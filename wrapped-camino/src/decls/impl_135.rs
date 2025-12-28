macro_rules! deps {
    () => {
        Utf8PathBuf!();
    };
}

macro_rules! impl_135 {
    () => {
        deps!();
        impl PartialOrd for Utf8PathBuf { # [inline] fn partial_cmp (& self , other : & Utf8PathBuf) -> Option < Ordering > { Some (self . cmp (other)) } }
    };
}

impl_135!()