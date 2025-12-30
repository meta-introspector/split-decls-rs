// Generated macro for impl_6 (impl)
macro_rules! Depcrateimpl_6 {
() => {
// Module: crate
// Provides: {"impl_6"}
// Dependencies: {}
unsafe impl GlobalAlloc for Counter { unsafe fn alloc (& self , layout : Layout) -> * mut u8 { let ret = System . alloc (layout) ; if ! ret . is_null () { ALLOCATED . fetch_add (layout . size () , Relaxed) ; } ret } unsafe fn dealloc (& self , ptr : * mut u8 , layout : Layout) { System . dealloc (ptr , layout) ; ALLOCATED . fetch_sub (layout . size () , Relaxed) ; } }
};
}
