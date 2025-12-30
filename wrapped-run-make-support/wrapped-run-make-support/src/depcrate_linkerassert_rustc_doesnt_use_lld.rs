// Generated macro for assert_rustc_doesnt_use_lld (function)
macro_rules! Depcrate_linkerassert_rustc_doesnt_use_lld {
() => {
// Module: crate::linker
// Provides: {"assert_rustc_doesnt_use_lld"}
// Dependencies: {}
# [doc = " Asserts that `rustc` doesn't use LLD for linking when executed."] pub fn assert_rustc_doesnt_use_lld (rustc : & mut Rustc) { let stderr = get_stderr_with_linker_messages (rustc) ; assert ! (! has_lld_version_in_logs (& stderr) , "LLD version should NOT be present in rustc stderr:\n{stderr}") ; }
};
}
