// Generated macro for stash_apply_progress_cb (function)
macro_rules! Depcrate_stashstash_apply_progress_cb {
() => {
// Module: crate::stash
// Provides: {"stash_apply_progress_cb"}
// Dependencies: {}
extern "C" fn stash_apply_progress_cb (progress : raw :: git_stash_apply_progress_t , payload : * mut c_void ,) -> c_int { panic :: wrap (| | unsafe { let options = & mut * (payload as * mut StashApplyOptions < '_ >) ; let res = { let callback = options . progress . as_mut () . unwrap () ; callback (convert_progress (progress)) } ; if res { 0 } else { - 1 } }) . unwrap_or (- 1) }
};
}
