// Generated macro for impl_95 (impl)
macro_rules! Depcrate_ext_sliceimpl_95 {
() => {
// Module: crate::ext_slice
// Provides: {"impl_95"}
// Dependencies: {}
impl < 'h , 'n > Find < 'h , 'n > { fn new (haystack : & 'h [u8] , needle : & 'n [u8]) -> Find < 'h , 'n > { Find { it : memmem :: find_iter (haystack , needle) , haystack , needle } } }
};
}
