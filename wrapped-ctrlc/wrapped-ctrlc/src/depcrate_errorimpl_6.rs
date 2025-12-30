// Generated macro for impl_6 (impl)
macro_rules! Depcrate_errorimpl_6 {
() => {
// Module: crate::error
// Provides: {"impl_6"}
// Dependencies: {}
impl From < platform :: Error > for Error { fn from (e : platform :: Error) -> Error { # [cfg (not (windows))] if e == platform :: Error :: EEXIST { return Error :: MultipleHandlers ; } let system_error = std :: io :: Error :: new (std :: io :: ErrorKind :: Other , e) ; Error :: System (system_error) } }
};
}
