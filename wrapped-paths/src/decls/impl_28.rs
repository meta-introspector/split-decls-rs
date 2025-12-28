macro_rules! deps {
    () => {
        RelPathBuf!();
    };
}

macro_rules! impl_28 {
    () => {
        deps!();
        impl TryFrom < Utf8PathBuf > for RelPathBuf { type Error = Utf8PathBuf ; fn try_from (path_buf : Utf8PathBuf) -> Result < RelPathBuf , Utf8PathBuf > { if ! path_buf . is_relative () { return Err (path_buf) ; } Ok (RelPathBuf (path_buf)) } }
    };
}

impl_28!()