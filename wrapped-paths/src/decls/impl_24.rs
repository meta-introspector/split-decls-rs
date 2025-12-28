macro_rules! deps {
    () => {
        RelPathBuf!();
    };
}

macro_rules! impl_24 {
    () => {
        deps!();
        impl From < RelPathBuf > for Utf8PathBuf { fn from (RelPathBuf (path_buf) : RelPathBuf) -> Utf8PathBuf { path_buf } }
    };
}

impl_24!();