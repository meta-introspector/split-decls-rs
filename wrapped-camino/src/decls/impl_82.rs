macro_rules! deps {
    () => {
        Utf8PathBuf!();
        FromPathBufError!();
        FromPathError!();
    };
}

macro_rules! impl_82 {
    () => {
        deps!();
        impl TryFrom < PathBuf > for Utf8PathBuf { type Error = FromPathBufError ; fn try_from (path : PathBuf) -> Result < Utf8PathBuf , Self :: Error > { Utf8PathBuf :: from_path_buf (path) . map_err (| path | FromPathBufError { path , error : FromPathError (()) , }) } }
    };
}

impl_82!()