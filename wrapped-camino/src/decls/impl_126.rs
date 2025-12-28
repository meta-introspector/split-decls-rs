macro_rules! deps {
    () => {
        Utf8PathBuf!();
    };
}

macro_rules! impl_126 {
    () => {
        deps!();
        impl AsRef < str > for Utf8PathBuf { # [inline] fn as_ref (& self) -> & str { self . as_str () } }
    };
}

impl_126!()