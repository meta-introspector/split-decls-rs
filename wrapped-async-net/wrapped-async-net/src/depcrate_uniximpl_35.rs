// Generated macro for impl_35 (impl)
macro_rules! Depcrate_uniximpl_35 {
() => {
// Module: crate::unix
// Provides: {"impl_35"}
// Dependencies: {}
impl From < UnixStream > for Arc < Async < std :: os :: unix :: net :: UnixStream > > { fn from (val : UnixStream) -> Self { val . inner } }
};
}
