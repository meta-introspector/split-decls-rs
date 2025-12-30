// Generated macro for impl_33 (impl)
macro_rules! Depcrate_errorimpl_33 {
() => {
// Module: crate::error
// Provides: {"impl_33"}
// Dependencies: {}
impl From < ShareError > for io :: Error { fn from (e : ShareError) -> io :: Error { io :: Error :: new (io :: ErrorKind :: Other , e) } }
};
}
