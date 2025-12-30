// Generated macro for free_boxed_slice (function)
macro_rules! Depcrate_bytesfree_boxed_slice {
() => {
// Module: crate::bytes
// Provides: {"free_boxed_slice"}
// Dependencies: {}
unsafe fn free_boxed_slice (buf : * mut u8 , offset : * const u8 , len : usize) { let cap = offset . offset_from (buf) as usize + len ; dealloc (buf , Layout :: from_size_align (cap , 1) . unwrap ()) }
};
}
