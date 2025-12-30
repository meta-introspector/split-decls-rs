// Generated macro for impl_36 (impl)
macro_rules! Depcrate_raw_vecimpl_36 {
() => {
// Module: crate::raw_vec
// Provides: {"impl_36"}
// Dependencies: {}
unsafe impl < # [may_dangle] T , A : Allocator > Drop for RawVec < T , A > { # [doc = " Frees the memory owned by the `RawVec` *without* trying to drop its contents."] fn drop (& mut self) { unsafe { self . inner . deallocate (T :: LAYOUT) } } }
};
}
