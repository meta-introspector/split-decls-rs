// Generated macro for ptr_map (function)
macro_rules! Depcrate_bytesptr_map {
() => {
// Module: crate::bytes
// Provides: {"ptr_map"}
// Dependencies: {}
# [cfg (not (miri))] fn ptr_map < F > (ptr : * mut u8 , f : F) -> * mut u8 where F : FnOnce (usize) -> usize , { let old_addr = ptr as usize ; let new_addr = f (old_addr) ; new_addr as * mut u8 }
};
}
