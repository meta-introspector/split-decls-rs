// Generated macro for impl_72 (impl)
macro_rules! Depcrate_errorsimpl_72 {
() => {
// Module: crate::errors
// Provides: {"impl_72"}
// Dependencies: {}
# [cfg (feature = "std")] impl From < Error > for std :: io :: Error { fn from (err : Error) -> std :: io :: Error { std :: io :: Error :: new (std :: io :: ErrorKind :: InvalidData , err) } }
};
}
