macro_rules! deps {
    () => {
        FromPathError!();
        FromPathBufError!();
        Utf8PathBuf!();
    };
}

macro_rules! impl_99 {
    () => {
        deps!();
        impl TryFrom < PathBuf > for Utf8PathBuf { type Error = FromPathBufError ; fn try_from (path : PathBuf) -> Result < Utf8PathBuf , Self :: Error > { Utf8PathBuf :: from_path_buf (path) . map_err (| path | FromPathBufError { path , error : FromPathError (()) , }) } }
    };
}

impl_99!();