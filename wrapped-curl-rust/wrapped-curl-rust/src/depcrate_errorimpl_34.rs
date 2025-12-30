// Generated macro for impl_34 (impl)
macro_rules! Depcrate_errorimpl_34 {
() => {
// Module: crate::error
// Provides: {"impl_34"}
// Dependencies: {}
impl From < MultiError > for io :: Error { fn from (e : MultiError) -> io :: Error { io :: Error :: new (io :: ErrorKind :: Other , e) } }
};
}
