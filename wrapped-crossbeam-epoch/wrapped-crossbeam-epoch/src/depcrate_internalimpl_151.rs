// Generated macro for impl_151 (impl)
macro_rules! Depcrate_internalimpl_151 {
() => {
// Module: crate::internal
// Provides: {"impl_151"}
// Dependencies: {}
impl Drop for Bag { fn drop (& mut self) { for deferred in & mut self . deferreds [.. self . len] { let no_op = Deferred :: NO_OP ; let owned_deferred = mem :: replace (deferred , no_op) ; owned_deferred . call () ; } } }
};
}
