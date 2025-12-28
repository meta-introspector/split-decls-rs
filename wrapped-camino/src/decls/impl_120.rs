macro_rules! deps {
    () => {
        Utf8Path!();
        Utf8PathBuf!();
    };
}

macro_rules! impl_120 {
    () => {
        deps!();
        impl AsRef < Utf8Path > for Utf8PathBuf { # [inline] fn as_ref (& self) -> & Utf8Path { self . as_path () } }
    };
}

impl_120!()