// Generated macro for impl_158 (impl)
macro_rules! Depcrate_errorimpl_158 {
() => {
// Module: crate::error
// Provides: {"impl_158"}
// Dependencies: {}
# [cfg (feature = "std")] impl From < std :: io :: Error > for Error { fn from (err : std :: io :: Error) -> Error { match err . kind () { std :: io :: ErrorKind :: NotFound => ErrorKind :: FileNotFound , std :: io :: ErrorKind :: PermissionDenied => ErrorKind :: PermissionDenied , other => ErrorKind :: Io (other) , } . into () } }
};
}
