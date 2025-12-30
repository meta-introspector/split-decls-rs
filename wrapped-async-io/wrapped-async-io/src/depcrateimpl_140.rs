// Generated macro for impl_140 (impl)
macro_rules! Depcrateimpl_140 {
() => {
// Module: crate
// Provides: {"impl_140"}
// Dependencies: {}
# [cfg (unix)] impl TryFrom < std :: os :: unix :: net :: UnixListener > for Async < std :: os :: unix :: net :: UnixListener > { type Error = io :: Error ; fn try_from (listener : std :: os :: unix :: net :: UnixListener) -> io :: Result < Self > { Async :: new (listener) } }
};
}
