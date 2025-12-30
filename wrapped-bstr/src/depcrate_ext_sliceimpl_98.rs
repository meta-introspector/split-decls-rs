// Generated macro for impl_98 (impl)
macro_rules! Depcrate_ext_sliceimpl_98 {
() => {
// Module: crate::ext_slice
// Provides: {"impl_98"}
// Dependencies: {}
impl < 'h , 'n > FindReverse < 'h , 'n > { fn new (haystack : & 'h [u8] , needle : & 'n [u8]) -> FindReverse < 'h , 'n > { FindReverse { it : memmem :: rfind_iter (haystack , needle) , haystack , needle , } } fn haystack (& self) -> & 'h [u8] { self . haystack } fn needle (& self) -> & 'n [u8] { self . needle } }
};
}
