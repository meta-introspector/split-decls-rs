// Generated macro for usize (function)
macro_rules! Depcrate_export_integersusize {
() => {
// Module: crate::export::integers
// Provides: {"usize"}
// Dependencies: {}
# [doc = " Implementation detail"] pub fn usize (b : & usize) { write (& (* b as u32) . to_le_bytes ()) }
};
}
