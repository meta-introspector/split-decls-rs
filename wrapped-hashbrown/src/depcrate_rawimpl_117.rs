// Generated macro for impl_117 (impl)
macro_rules! Depcrate_rawimpl_117 {
() => {
// Module: crate::raw
// Provides: {"impl_117"}
// Dependencies: {}
# [cfg (not (feature = "nightly"))] impl < T , A : Allocator > Drop for RawIntoIter < T , A > { # [cfg_attr (feature = "inline-more" , inline)] fn drop (& mut self) { unsafe { self . iter . drop_elements () ; if let Some ((ptr , layout , ref alloc)) = self . allocation { alloc . deallocate (ptr , layout) ; } } } }
};
}
