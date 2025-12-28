macro_rules! deps {
    () => {
        Utf8PathBuf!();
        Utf8Path!();
    };
}

macro_rules! impl_5 {
    () => {
        deps!();
        impl Deref for Utf8PathBuf { type Target = Utf8Path ; fn deref (& self) -> & Utf8Path { self . as_path () } }
    };
}

impl_5!()