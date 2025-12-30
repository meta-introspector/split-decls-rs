// Generated macro for impl_113 (impl)
macro_rules! Depcrate_ext_sliceimpl_113 {
() => {
// Module: crate::ext_slice
// Provides: {"impl_113"}
// Dependencies: {}
impl < 'h , 's > Split < 'h , 's > { fn new (haystack : & 'h [u8] , splitter : & 's [u8]) -> Split < 'h , 's > { let finder = haystack . find_iter (splitter) ; Split { finder , last : 0 , done : false } } }
};
}
