macro_rules! deps {
    () => {
        Utf8PathBuf!();
    };
}

macro_rules! impl_95 {
    () => {
        deps!();
        impl From < Utf8PathBuf > for Box < Path > { fn from (path : Utf8PathBuf) -> Box < Path > { PathBuf :: from (path) . into_boxed_path () } }
    };
}

impl_95!()