macro_rules! deps {
    () => {
        Utf8Path!();
        Utf8PathBuf!();
    };
}

macro_rules! impl_87 {
    () => {
        deps!();
        impl From < Utf8PathBuf > for Box < Utf8Path > { fn from (path : Utf8PathBuf) -> Box < Utf8Path > { path . into_boxed_path () } }
    };
}

impl_87!();