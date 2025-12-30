// Generated macro for impl_18 (impl)
macro_rules! Depcrate_uniximpl_18 {
() => {
// Module: crate::unix
// Provides: {"impl_18"}
// Dependencies: {}
impl TryFrom < std :: os :: unix :: net :: UnixListener > for UnixListener { type Error = io :: Error ; fn try_from (listener : std :: os :: unix :: net :: UnixListener) -> io :: Result < UnixListener > { Ok (UnixListener :: new (Arc :: new (Async :: new (listener) ?))) } }
};
}
