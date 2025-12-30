// Generated macro for linker_args (function)
macro_rules! Depcrate_utils_helperslinker_args {
() => {
// Module: crate::utils::helpers
// Provides: {"linker_args"}
// Dependencies: {}
# [doc = " Returns the linker arguments for rustc/rustdoc for the given builder and target."] pub fn linker_args (builder : & Builder < '_ > , target : TargetSelection , lld_threads : LldThreads ,) -> Vec < String > { let mut args = linker_flags (builder , target , lld_threads) ; if let Some (linker) = builder . linker (target) { args . push (format ! ("-Clinker={}" , linker . display ())) ; } args }
};
}
