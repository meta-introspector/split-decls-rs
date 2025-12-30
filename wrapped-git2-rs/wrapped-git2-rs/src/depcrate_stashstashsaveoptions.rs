// Generated macro for StashSaveOptions (struct)
macro_rules! Depcrate_stashStashSaveOptions {
() => {
// Module: crate::stash
// Provides: {"StashSaveOptions"}
// Dependencies: {}
# [doc = " Stash application options structure"] pub struct StashSaveOptions < 'a > { message : Option < CString > , flags : Option < StashFlags > , stasher : Signature < 'a > , pathspec : Vec < CString > , pathspec_ptrs : Vec < * const c_char > , raw_opts : raw :: git_stash_save_options , }
};
}
