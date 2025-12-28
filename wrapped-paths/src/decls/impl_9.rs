macro_rules! deps {
    () => {
        AbsPathBuf!();
    };
}

macro_rules! impl_9 {
    () => {
        deps!();
        impl TryFrom < Utf8PathBuf > for AbsPathBuf { type Error = Utf8PathBuf ; fn try_from (path_buf : Utf8PathBuf) -> Result < AbsPathBuf , Utf8PathBuf > { if ! path_buf . is_absolute () { return Err (path_buf) ; } Ok (AbsPathBuf (path_buf)) } }
    };
}

impl_9!();