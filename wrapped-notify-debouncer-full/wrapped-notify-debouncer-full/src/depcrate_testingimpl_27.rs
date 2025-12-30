// Generated macro for impl_27 (impl)
macro_rules! Depcrate_testingimpl_27 {
() => {
// Module: crate::testing
// Provides: {"impl_27"}
// Dependencies: {}
impl schema :: Error { pub fn into_notify_error (self) -> Error { let kind = match & * self . kind { "path-not-found" => ErrorKind :: PathNotFound , "watch-not-found" => ErrorKind :: WatchNotFound , "max-files-watch" => ErrorKind :: MaxFilesWatch , _ => panic ! ("unknown error type `{}`" , self . kind) , } ; let mut error = Error :: new (kind) ; for p in self . paths { error = error . add_path (PathBuf :: from (p)) ; } error } }
};
}
