// Generated macro for isize (function)
macro_rules! Depcrate_export_integersisize {
() => {
// Module: crate::export::integers
// Provides: {"isize"}
// Dependencies: {}
# [doc = " Implementation detail"] pub fn isize (b : & isize) { write (& (* b as i32) . to_le_bytes ()) }
};
}
