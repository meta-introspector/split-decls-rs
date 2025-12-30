// Generated macro for impl_116 (impl)
macro_rules! Depcrate_ext_sliceimpl_116 {
() => {
// Module: crate::ext_slice
// Provides: {"impl_116"}
// Dependencies: {}
impl < 'h , 's > SplitReverse < 'h , 's > { fn new (haystack : & 'h [u8] , splitter : & 's [u8]) -> SplitReverse < 'h , 's > { let finder = haystack . rfind_iter (splitter) ; SplitReverse { finder , last : haystack . len () , done : false } } }
};
}
