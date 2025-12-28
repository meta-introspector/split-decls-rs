macro_rules! deps {
    () => {
        AbsPathBuf!();
    };
}

macro_rules! impl_2 {
    () => {
        deps!();
        impl From < AbsPathBuf > for PathBuf { fn from (AbsPathBuf (path_buf) : AbsPathBuf) -> PathBuf { path_buf . into () } }
    };
}

impl_2!()