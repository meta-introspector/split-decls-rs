// Generated macro for impl_33 (impl)
macro_rules! Depcrate_uniximpl_33 {
() => {
// Module: crate::unix
// Provides: {"impl_33"}
// Dependencies: {}
impl From < Async < std :: os :: unix :: net :: UnixStream > > for UnixStream { fn from (stream : Async < std :: os :: unix :: net :: UnixStream >) -> UnixStream { UnixStream :: new (Arc :: new (stream)) } }
};
}
