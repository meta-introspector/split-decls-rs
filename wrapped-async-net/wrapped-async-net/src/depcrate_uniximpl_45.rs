// Generated macro for impl_45 (impl)
macro_rules! Depcrate_uniximpl_45 {
() => {
// Module: crate::unix
// Provides: {"impl_45"}
// Dependencies: {}
impl TryFrom < std :: os :: unix :: net :: UnixDatagram > for UnixDatagram { type Error = io :: Error ; fn try_from (socket : std :: os :: unix :: net :: UnixDatagram) -> io :: Result < UnixDatagram > { Ok (UnixDatagram :: new (Arc :: new (Async :: new (socket) ?))) } }
};
}
