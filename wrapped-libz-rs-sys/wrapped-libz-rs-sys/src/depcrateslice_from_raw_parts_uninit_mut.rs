// Generated macro for slice_from_raw_parts_uninit_mut (function)
macro_rules! Depcrateslice_from_raw_parts_uninit_mut {
() => {
// Module: crate
// Provides: {"slice_from_raw_parts_uninit_mut"}
// Dependencies: {}
# [doc = " # Safety"] # [doc = ""] # [doc = " Either"] # [doc = ""] # [doc = " - `ptr` is `NULL`"] # [doc = " - `ptr` and `len` satisfy the requirements of [`core::slice::from_raw_parts_mut`]"] unsafe fn slice_from_raw_parts_uninit_mut < 'a , T > (ptr : * mut T , len : usize ,) -> Option < & 'a mut [MaybeUninit < T >] > { if ptr . is_null () { None } else { Some (unsafe { core :: slice :: from_raw_parts_mut (ptr . cast :: < MaybeUninit < T > > () , len) }) } }
};
}
