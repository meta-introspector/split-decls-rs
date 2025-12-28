macro_rules! deps {
    () => {
        Utf8PathBuf!();
        Utf8Path!();
    };
}

macro_rules! impl_70 {
    () => {
        deps!();
        impl From < Utf8PathBuf > for Box < Utf8Path > { fn from (path : Utf8PathBuf) -> Box < Utf8Path > { path . into_boxed_path () } }
    };
}

impl_70!()