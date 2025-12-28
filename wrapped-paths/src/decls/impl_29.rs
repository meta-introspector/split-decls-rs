macro_rules! deps {
    () => {
        RelPathBuf!();
    };
}

macro_rules! impl_29 {
    () => {
        deps!();
        impl TryFrom < & str > for RelPathBuf { type Error = Utf8PathBuf ; fn try_from (path : & str) -> Result < RelPathBuf , Utf8PathBuf > { RelPathBuf :: try_from (Utf8PathBuf :: from (path)) } }
    };
}

impl_29!()