// Generated macro for impl_142 (impl)
macro_rules! Depcrateimpl_142 {
() => {
// Module: crate
// Provides: {"impl_142"}
// Dependencies: {}
# [cfg (unix)] impl TryFrom < std :: os :: unix :: net :: UnixStream > for Async < std :: os :: unix :: net :: UnixStream > { type Error = io :: Error ; fn try_from (stream : std :: os :: unix :: net :: UnixStream) -> io :: Result < Self > { Async :: new (stream) } }
};
}
