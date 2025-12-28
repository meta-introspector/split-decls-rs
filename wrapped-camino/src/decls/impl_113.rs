macro_rules! deps {
    () => {
        Utf8PathBuf!();
        Utf8Path!();
    };
}

macro_rules! impl_113 {
    () => {
        deps!();
        impl ToOwned for Utf8Path { type Owned = Utf8PathBuf ; # [inline] fn to_owned (& self) -> Utf8PathBuf { self . to_path_buf () } }
    };
}

impl_113!()