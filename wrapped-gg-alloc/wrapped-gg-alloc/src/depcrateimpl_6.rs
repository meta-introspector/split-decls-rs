// Generated macro for impl_6 (impl)
macro_rules! Depcrateimpl_6 {
() => {
// Module: crate
// Provides: {"impl_6"}
// Dependencies: {}
unsafe impl < A > GlobalAlloc for GgAlloc < A > where A : GlobalAlloc , { unsafe fn alloc (& self , layout : Layout) -> * mut u8 { let mut ret = self . alloc . alloc (layout) ; loop { if ret . is_null () { break ; } if pointer_above_2g (ret) { break ; } let mut size = 1 << 27 ; loop { let test_layout = Layout :: from_size_align (size , 1) . unwrap () ; let fill_ptr = self . alloc . alloc (test_layout) ; if ! fill_ptr . is_null () && alloc_fully_below_2g (fill_ptr , test_layout) { } else { if ! fill_ptr . is_null () { self . alloc . dealloc (fill_ptr , test_layout) ; } size /= 2 ; if size < 1 { break ; } } } if ! alloc_fully_below_2g (ret , layout) { self . alloc . dealloc (ret , layout) ; } loop { let test_layout = layout ; let fill_ptr = self . alloc . alloc (test_layout) ; if ! fill_ptr . is_null () && alloc_fully_below_2g (fill_ptr , test_layout) { } else { if pointer_above_2g (fill_ptr) { return fill_ptr ; } else { break ; } } } ret = self . alloc . alloc (layout) ; } ret } unsafe fn dealloc (& self , ptr : * mut u8 , layout : Layout) { self . alloc . dealloc (ptr , layout) ; } }
};
}
