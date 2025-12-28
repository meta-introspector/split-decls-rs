macro_rules! deps {
    () => {
        AbsPathBuf!();
    };
}

macro_rules! impl_1 {
    () => {
        deps!();
        impl From < AbsPathBuf > for Utf8PathBuf { fn from (AbsPathBuf (path_buf) : AbsPathBuf) -> Utf8PathBuf { path_buf } }
    };
}

impl_1!();