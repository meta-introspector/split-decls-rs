macro_rules! deps {
    () => {
        Utf8PathBuf!();
        Utf8Path!();
    };
}

macro_rules! impl_129 {
    () => {
        deps!();
        impl Borrow < Utf8Path > for Utf8PathBuf { # [inline] fn borrow (& self) -> & Utf8Path { self . as_path () } }
    };
}

impl_129!()