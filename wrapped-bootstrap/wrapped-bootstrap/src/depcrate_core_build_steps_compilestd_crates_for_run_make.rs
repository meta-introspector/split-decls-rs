// Generated macro for std_crates_for_run_make (function)
macro_rules! Depcrate_core_build_steps_compilestd_crates_for_run_make {
() => {
// Module: crate::core::build_steps::compile
// Provides: {"std_crates_for_run_make"}
// Dependencies: {}
# [doc = " Resolves standard library crates for `Std::run_make` for any build kind (like check, doc,"] # [doc = " build, clippy, etc.)."] pub fn std_crates_for_run_make (run : & RunConfig < '_ >) -> Vec < String > { let mut crates = run . make_run_crates (builder :: Alias :: Library) ; let target_is_no_std = run . builder . no_std (run . target) . unwrap_or (false) ; if target_is_no_std { crates . retain (| c | c == "core" || c == "alloc") ; } crates }
};
}
