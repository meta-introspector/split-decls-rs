macro_rules! deps {
    () => {
        Utf8PathBuf!();
    };
}

macro_rules! impl_79 {
    () => {
        deps!();
        impl From < Utf8PathBuf > for Arc < Path > { fn from (path : Utf8PathBuf) -> Arc < Path > { PathBuf :: from (path) . into () } }
    };
}

impl_79!()