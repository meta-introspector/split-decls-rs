// Generated macro for impl_34 (impl)
macro_rules! Depcrate_uniximpl_34 {
() => {
// Module: crate::unix
// Provides: {"impl_34"}
// Dependencies: {}
impl TryFrom < std :: os :: unix :: net :: UnixStream > for UnixStream { type Error = io :: Error ; fn try_from (stream : std :: os :: unix :: net :: UnixStream) -> io :: Result < UnixStream > { Ok (UnixStream :: new (Arc :: new (Async :: new (stream) ?))) } }
};
}
