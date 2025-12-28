macro_rules! deps {
    () => {
        Utf8Path!();
        Utf8PathBuf!();
    };
}

macro_rules! impl_22 {
    () => {
        deps!();
        impl Deref for Utf8PathBuf { type Target = Utf8Path ; fn deref (& self) -> & Utf8Path { self . as_path () } }
    };
}

impl_22!()