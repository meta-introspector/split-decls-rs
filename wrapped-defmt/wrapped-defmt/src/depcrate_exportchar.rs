// Generated macro for char (function)
macro_rules! Depcrate_exportchar {
() => {
// Module: crate::export
// Provides: {"char"}
// Dependencies: {}
# [doc = " Implementation detail"] pub fn char (b : & char) { write (& (* b as u32) . to_le_bytes ()) }
};
}
