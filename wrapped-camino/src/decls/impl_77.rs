macro_rules! deps {
    () => {
        Utf8PathBuf!();
    };
}

macro_rules! impl_77 {
    () => {
        deps!();
        impl From < Utf8PathBuf > for PathBuf { fn from (path : Utf8PathBuf) -> PathBuf { path . 0 } }
    };
}

impl_77!()