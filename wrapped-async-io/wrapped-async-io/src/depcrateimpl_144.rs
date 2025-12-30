// Generated macro for impl_144 (impl)
macro_rules! Depcrateimpl_144 {
() => {
// Module: crate
// Provides: {"impl_144"}
// Dependencies: {}
# [cfg (unix)] impl TryFrom < std :: os :: unix :: net :: UnixDatagram > for Async < std :: os :: unix :: net :: UnixDatagram > { type Error = io :: Error ; fn try_from (socket : std :: os :: unix :: net :: UnixDatagram) -> io :: Result < Self > { Async :: new (socket) } }
};
}
