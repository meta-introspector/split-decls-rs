// Generated macro for impl_213 (impl)
macro_rules! Depcrate_internalimpl_213 {
() => {
// Module: crate::internal
// Provides: {"impl_213"}
// Dependencies: {}
impl < T , N : ArrayLength > Drop for IntrusiveArrayConsumer < '_ , T , N > { fn drop (& mut self) { unsafe { ptr :: drop_in_place (self . array . get_unchecked_mut (self . position ..)) ; } } }
};
}
