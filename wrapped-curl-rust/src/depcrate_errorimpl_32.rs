// Generated macro for impl_32 (impl)
macro_rules! Depcrate_errorimpl_32 {
() => {
// Module: crate::error
// Provides: {"impl_32"}
// Dependencies: {}
impl From < Error > for io :: Error { fn from (e : Error) -> io :: Error { io :: Error :: new (io :: ErrorKind :: Other , e) } }
};
}
