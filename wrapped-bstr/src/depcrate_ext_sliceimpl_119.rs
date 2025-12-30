// Generated macro for impl_119 (impl)
macro_rules! Depcrate_ext_sliceimpl_119 {
() => {
// Module: crate::ext_slice
// Provides: {"impl_119"}
// Dependencies: {}
impl < 'h , 's > SplitN < 'h , 's > { fn new (haystack : & 'h [u8] , splitter : & 's [u8] , limit : usize ,) -> SplitN < 'h , 's > { let split = haystack . split_str (splitter) ; SplitN { split , limit , count : 0 } } }
};
}
