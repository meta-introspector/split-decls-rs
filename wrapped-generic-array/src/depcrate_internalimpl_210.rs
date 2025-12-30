// Generated macro for impl_210 (impl)
macro_rules! Depcrate_internalimpl_210 {
() => {
// Module: crate::internal
// Provides: {"impl_210"}
// Dependencies: {}
impl < T , N : ArrayLength > Drop for ArrayConsumer < T , N > { fn drop (& mut self) { unsafe { ptr :: drop_in_place (self . array . get_unchecked_mut (self . position ..)) ; } } }
};
}
