macro_rules! deps {
    () => {
        Utf8PathBuf!();
    };
}

macro_rules! impl_80 {
    () => {
        deps!();
        impl From < Utf8PathBuf > for Rc < Path > { fn from (path : Utf8PathBuf) -> Rc < Path > { PathBuf :: from (path) . into () } }
    };
}

impl_80!()