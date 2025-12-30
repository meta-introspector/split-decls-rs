// Generated macro for ref_slice_mut (function)
macro_rules! Depcrateref_slice_mut {
() => {
// Module: crate
// Provides: {"ref_slice_mut"}
// Dependencies: {}
# [doc = " Create a length 1 mutable slice out of a reference"] pub fn ref_slice_mut < T > (ptr : & mut T) -> & mut [T] { unsafe { std :: slice :: from_raw_parts_mut (ptr , 1) } }
};
}
