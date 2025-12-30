// Generated macro for raw_data (function)
macro_rules! Depcrate_ffi_utilraw_data {
() => {
// Module: crate::ffi_util
// Provides: {"raw_data"}
// Dependencies: {}
pub (crate) unsafe fn raw_data (ptr : * const c_char , size : usize) -> Option < Vec < u8 > > { if ptr . is_null () { None } else { let mut dst = vec ! [0 ; size] ; unsafe { ptr :: copy_nonoverlapping (ptr as * const u8 , dst . as_mut_ptr () , size) } ; Some (dst) } }
};
}
