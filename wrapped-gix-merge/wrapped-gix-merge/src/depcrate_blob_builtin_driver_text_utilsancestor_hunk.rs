// Generated macro for ancestor_hunk (function)
macro_rules! Depcrate_blob_builtin_driver_text_utilsancestor_hunk {
() => {
// Module: crate::blob::builtin_driver::text::utils
// Provides: {"ancestor_hunk"}
// Dependencies: {}
fn ancestor_hunk (start : u32 , num_lines : u32) -> Hunk { let range = start .. start + num_lines ; Hunk { before : range . clone () , after : range , side : Side :: Ancestor , } }
};
}
