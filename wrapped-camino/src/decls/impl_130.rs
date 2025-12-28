macro_rules! deps {
    () => {
        Utf8Path!();
        Utf8PathBuf!();
    };
}

macro_rules! impl_130 {
    () => {
        deps!();
        impl ToOwned for Utf8Path { type Owned = Utf8PathBuf ; # [inline] fn to_owned (& self) -> Utf8PathBuf { self . to_path_buf () } }
    };
}

impl_130!()