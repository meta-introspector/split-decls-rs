// Generated macro for write_to_le_bytes (macro)
macro_rules! Depcrate_export_integerswrite_to_le_bytes {
() => {
// Module: crate::export::integers
// Provides: {"write_to_le_bytes"}
// Dependencies: {}
macro_rules ! write_to_le_bytes { ($ ($ s : ident) ,*) => { $ (# [doc = " Implementation detail"] pub fn $ s (b : &$ s) { write (& b . to_le_bytes ()) }) * } ; }
};
}
