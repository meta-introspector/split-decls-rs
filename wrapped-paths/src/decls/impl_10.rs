macro_rules! deps {
    () => {
        AbsPathBuf!();
    };
}

macro_rules! impl_10 {
    () => {
        deps!();
        impl TryFrom < & str > for AbsPathBuf { type Error = Utf8PathBuf ; fn try_from (path : & str) -> Result < AbsPathBuf , Utf8PathBuf > { AbsPathBuf :: try_from (Utf8PathBuf :: from (path)) } }
    };
}

impl_10!()