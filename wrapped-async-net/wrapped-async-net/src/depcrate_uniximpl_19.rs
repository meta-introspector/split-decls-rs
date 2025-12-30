// Generated macro for impl_19 (impl)
macro_rules! Depcrate_uniximpl_19 {
() => {
// Module: crate::unix
// Provides: {"impl_19"}
// Dependencies: {}
impl From < UnixListener > for Arc < Async < std :: os :: unix :: net :: UnixListener > > { fn from (val : UnixListener) -> Self { val . inner } }
};
}
