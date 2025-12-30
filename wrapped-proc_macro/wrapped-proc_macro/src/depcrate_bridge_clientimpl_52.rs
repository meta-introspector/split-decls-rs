// Generated macro for impl_52 (impl)
macro_rules! Depcrate_bridge_clientimpl_52 {
() => {
// Module: crate::bridge::client
// Provides: {"impl_52"}
// Dependencies: {}
impl Bridge < '_ > { fn with < R > (f : impl FnOnce (& mut Bridge < '_ >) -> R) -> R { state :: with (| state | { let bridge = state . expect ("procedural macro API is used outside of a procedural macro") ; let mut bridge = bridge . try_borrow_mut () . expect ("procedural macro API is used while it's already in use") ; f (& mut bridge) }) } }
};
}
