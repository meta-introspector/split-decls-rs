// Generated macro for impl_35 (impl)
macro_rules! Depcrate_errorimpl_35 {
() => {
// Module: crate::error
// Provides: {"impl_35"}
// Dependencies: {}
impl From < FormError > for io :: Error { fn from (e : FormError) -> io :: Error { io :: Error :: new (io :: ErrorKind :: Other , e) } }
};
}
