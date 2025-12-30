// Generated macro for impl_142 (impl)
macro_rules! Depcrate_raw_vecimpl_142 {
() => {
// Module: crate::raw_vec
// Provides: {"impl_142"}
// Dependencies: {}
impl < T , A : Allocator > Drop for RawVec < T , A > { # [doc = " Frees the memory owned by the `RawVec` *without* trying to drop its contents."] # [inline (always)] fn drop (& mut self) { if let Some ((ptr , layout)) = self . current_memory () { unsafe { self . alloc . deallocate (ptr , layout) } } } }
};
}
