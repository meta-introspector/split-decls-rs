macro_rules! deps {
    () => {
        Utf8PathBuf!();
    };
}

macro_rules! impl_107 {
    () => {
        deps!();
        impl AsRef < Path > for Utf8PathBuf { # [inline] fn as_ref (& self) -> & Path { & self . 0 } }
    };
}

impl_107!()