// Generated macro for linker_flags (function)
macro_rules! Depcrate_utils_helperslinker_flags {
() => {
// Module: crate::utils::helpers
// Provides: {"linker_flags"}
// Dependencies: {}
# [doc = " Returns the linker arguments for rustc/rustdoc for the given builder and target, without the"] # [doc = " -Clinker flag."] pub fn linker_flags (builder : & Builder < '_ > , target : TargetSelection , lld_threads : LldThreads ,) -> Vec < String > { let mut args = vec ! [] ; if ! builder . is_lld_direct_linker (target) && builder . config . lld_mode . is_used () { match builder . config . lld_mode { LldMode :: External => { args . push ("-Clinker-features=+lld" . to_string ()) ; args . push ("-Zunstable-options" . to_string ()) ; } LldMode :: SelfContained => { args . push ("-Clinker-features=+lld" . to_string ()) ; args . push ("-Clink-self-contained=+linker" . to_string ()) ; args . push ("-Zunstable-options" . to_string ()) ; } LldMode :: Unused => unreachable ! () , } ; if matches ! (lld_threads , LldThreads :: No) { args . push (format ! ("-Clink-arg=-Wl,{}" , lld_flag_no_threads (builder , builder . config . lld_mode , target . is_windows ()))) ; } } args }
};
}
