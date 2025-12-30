// Generated macro for impl_428 (impl)
macro_rules! Depcrate_errorimpl_428 {
() => {
// Module: crate::error
// Provides: {"impl_428"}
// Dependencies: {}
impl From < ErrorStack > for io :: Error { fn from (e : ErrorStack) -> io :: Error { io :: Error :: new (io :: ErrorKind :: Other , e) } }
};
}
