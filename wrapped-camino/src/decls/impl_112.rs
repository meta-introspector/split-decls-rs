macro_rules! deps {
    () => {
        Utf8Path!();
        Utf8PathBuf!();
    };
}

macro_rules! impl_112 {
    () => {
        deps!();
        impl Borrow < Utf8Path > for Utf8PathBuf { # [inline] fn borrow (& self) -> & Utf8Path { self . as_path () } }
    };
}

impl_112!()