// Generated macro for impl_91 (impl)
macro_rules! Depcrate_errorimpl_91 {
() => {
// Module: crate::error
// Provides: {"impl_91"}
// Dependencies: {}
impl From < Error > for io :: Error { fn from (err : Error) -> io :: Error { io :: Error :: new (io :: ErrorKind :: Other , err) } }
};
}
