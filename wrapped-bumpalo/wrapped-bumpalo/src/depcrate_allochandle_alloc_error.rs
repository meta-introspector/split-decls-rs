// Generated macro for handle_alloc_error (function)
macro_rules! Depcrate_allochandle_alloc_error {
() => {
// Module: crate::alloc
// Provides: {"handle_alloc_error"}
// Dependencies: {}
pub fn handle_alloc_error (layout : Layout) -> ! { panic ! ("encountered allocation error: {:?}" , layout) }
};
}
