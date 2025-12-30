// Generated macro for DiffOptions (struct)
macro_rules! Depcrate_diffDiffOptions {
() => {
// Module: crate::diff
// Provides: {"DiffOptions"}
// Dependencies: {}
# [doc = " Structure describing options about how the diff should be executed."] pub struct DiffOptions { pathspec : Vec < CString > , pathspec_ptrs : Vec < * const c_char > , old_prefix : Option < CString > , new_prefix : Option < CString > , raw : raw :: git_diff_options , }
};
}
