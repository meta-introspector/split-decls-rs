// Generated macro for mode_is_dir (function)
macro_rules! Depcrate_stackmode_is_dir {
() => {
// Module: crate::stack
// Provides: {"mode_is_dir"}
// Dependencies: {}
fn mode_is_dir (mode : Option < gix_index :: entry :: Mode >) -> Option < bool > { mode . map (| m | m . is_sparse () || m . is_submodule ()) }
};
}
