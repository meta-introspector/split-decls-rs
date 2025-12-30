// Generated macro for ref_slice (function)
macro_rules! Depcrateref_slice {
() => {
// Module: crate
// Provides: {"ref_slice"}
// Dependencies: {}
# [doc = " Create a length 1 slice out of a reference"] pub fn ref_slice < T > (ptr : & T) -> & [T] { unsafe { std :: slice :: from_raw_parts (ptr , 1) } }
};
}
