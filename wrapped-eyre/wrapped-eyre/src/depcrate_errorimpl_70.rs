// Generated macro for impl_70 (impl)
macro_rules! Depcrate_errorimpl_70 {
() => {
// Module: crate::error
// Provides: {"impl_70"}
// Dependencies: {}
impl < E > ErrorImpl < E > { # [doc = " Returns a type erased Error"] fn erase (& self) -> RefPtr < '_ , ErrorImpl < () > > { RefPtr :: new (self) . cast () } }
};
}
