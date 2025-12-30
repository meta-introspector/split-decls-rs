// Generated macro for impl_554 (impl)
macro_rules! Depcrate_concurrency_vector_clockimpl_554 {
() => {
// Module: crate::concurrency::vector_clock
// Provides: {"impl_554"}
// Dependencies: {}
impl Clone for VClock { fn clone (& self) -> Self { VClock (self . 0 . clone ()) } fn clone_from (& mut self , source : & Self) { let source_slice = source . as_slice () ; self . 0 . clear () ; self . 0 . extend_from_slice (source_slice) ; } }
};
}
