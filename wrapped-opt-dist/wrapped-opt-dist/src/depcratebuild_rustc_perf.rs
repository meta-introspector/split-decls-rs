// Generated macro for build_rustc_perf (function)
macro_rules! Depcratebuild_rustc_perf {
() => {
// Module: crate
// Provides: {"build_rustc_perf"}
// Dependencies: {}
fn build_rustc_perf (env : & Environment) -> anyhow :: Result < () > { cmd (& [env . cargo_stage_0 () . as_str () , "build" , "-p" , "collector"]) . workdir (& env . rustc_perf_dir ()) . env ("RUSTC" , & env . rustc_stage_0 () . into_string ()) . env ("RUSTC_BOOTSTRAP" , "1") . run () ? ; Ok (()) }
};
}
