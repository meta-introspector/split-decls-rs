// Generated macro for impl_17 (impl)
macro_rules! Depcrateimpl_17 {
() => {
// Module: crate
// Provides: {"impl_17"}
// Dependencies: {}
unsafe impl < # [may_dangle] T > Drop for TypedArena < T > { fn drop (& mut self) { unsafe { let mut chunks_borrow = self . chunks . borrow_mut () ; if let Some (mut last_chunk) = chunks_borrow . pop () { self . clear_last_chunk (& mut last_chunk) ; for chunk in chunks_borrow . iter_mut () { chunk . destroy (chunk . entries) ; } } } } }
};
}
