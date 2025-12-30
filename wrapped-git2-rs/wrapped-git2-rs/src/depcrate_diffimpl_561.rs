// Generated macro for impl_561 (impl)
macro_rules! Depcrate_diffimpl_561 {
() => {
// Module: crate::diff
// Provides: {"impl_561"}
// Dependencies: {}
impl DiffPatchidOptions { # [doc = " Creates a new set of patchid options,"] # [doc = " initialized to the default values"] pub fn new () -> Self { let mut opts = DiffPatchidOptions { raw : unsafe { mem :: zeroed () } , } ; assert_eq ! (unsafe { raw :: git_diff_patchid_options_init (& mut opts . raw , raw :: GIT_DIFF_PATCHID_OPTIONS_VERSION ,) } , 0) ; opts } }
};
}
