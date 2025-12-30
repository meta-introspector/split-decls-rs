// Generated macro for impl_19 (impl)
macro_rules! Depcrate_oomimpl_19 {
() => {
// Module: crate::oom
// Provides: {"impl_19"}
// Dependencies: {}
unsafe impl GlobalAlloc for OOMAllocator { unsafe fn alloc (& self , layout : Layout) -> * mut u8 { panic_if (| | rand :: random :: < u32 > () . is_multiple_of (2 + PANIC_COUNT . load (Relaxed))) ; unsafe { System . alloc (layout) } } unsafe fn dealloc (& self , ptr : * mut u8 , layout : Layout) { unsafe { System . dealloc (ptr , layout) } } }
};
}
