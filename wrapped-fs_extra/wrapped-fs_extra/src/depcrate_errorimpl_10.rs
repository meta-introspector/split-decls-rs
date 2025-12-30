// Generated macro for impl_10 (impl)
macro_rules! Depcrate_errorimpl_10 {
() => {
// Module: crate::error
// Provides: {"impl_10"}
// Dependencies: {}
impl ErrorKind { fn as_str (& self) -> & str { match * self { ErrorKind :: NotFound => "entity not found" , ErrorKind :: PermissionDenied => "permission denied" , ErrorKind :: AlreadyExists => "entity already exists" , ErrorKind :: Interrupted => "operation interrupted" , ErrorKind :: Other => "other os error" , ErrorKind :: InvalidFolder => "invalid folder error" , ErrorKind :: InvalidFile => "invalid file error" , ErrorKind :: InvalidFileName => "invalid file name error" , ErrorKind :: InvalidPath => "invalid path error" , ErrorKind :: Io (_) => "Io error" , ErrorKind :: StripPrefix (_) => "Strip prefix error" , ErrorKind :: OsString (_) => "OsString error" , } } }
};
}
