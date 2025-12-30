// Generated macro for impl_57 (impl)
macro_rules! Depcrate_collections_raw_vecimpl_57 {
() => {
// Module: crate::collections::raw_vec
// Provides: {"impl_57"}
// Dependencies: {}
impl < 'a , T > RawVec < 'a , T > { # [doc = " Frees the memory owned by the RawVec *without* trying to Drop its contents."] pub unsafe fn dealloc_buffer (& mut self) { let elem_size = mem :: size_of :: < T > () ; if elem_size != 0 { if let Some (layout) = self . current_layout () { self . a . dealloc (self . ptr . cast () , layout) ; } } } }
};
}
