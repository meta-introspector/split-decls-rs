// Generated macro for impl_44 (impl)
macro_rules! Depcrate_uniximpl_44 {
() => {
// Module: crate::unix
// Provides: {"impl_44"}
// Dependencies: {}
impl From < Async < std :: os :: unix :: net :: UnixDatagram > > for UnixDatagram { fn from (socket : Async < std :: os :: unix :: net :: UnixDatagram >) -> UnixDatagram { UnixDatagram :: new (Arc :: new (socket)) } }
};
}
