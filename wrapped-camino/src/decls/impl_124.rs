macro_rules! deps {
    () => {
        Utf8PathBuf!();
    };
}

macro_rules! impl_124 {
    () => {
        deps!();
        impl AsRef < Path > for Utf8PathBuf { # [inline] fn as_ref (& self) -> & Path { & self . 0 } }
    };
}

impl_124!();