// Generated macro for impl_122 (impl)
macro_rules! Depcrate_ext_sliceimpl_122 {
() => {
// Module: crate::ext_slice
// Provides: {"impl_122"}
// Dependencies: {}
impl < 'h , 's > SplitNReverse < 'h , 's > { fn new (haystack : & 'h [u8] , splitter : & 's [u8] , limit : usize ,) -> SplitNReverse < 'h , 's > { let split = haystack . rsplit_str (splitter) ; SplitNReverse { split , limit , count : 0 } } }
};
}
