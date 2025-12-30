// Generated macro for init_compiler_benchmarks (function)
macro_rules! Depcrate_traininginit_compiler_benchmarks {
() => {
// Module: crate::training
// Provides: {"init_compiler_benchmarks"}
// Dependencies: {}
fn init_compiler_benchmarks (env : & Environment , profiles : & [& str] , scenarios : & [& str] , crates : & [& str] ,) -> CmdBuilder { let mut cmd = cmd (& [env . cargo_stage_0 () . as_str () , "run" , "-p" , "collector" , "--bin" , "collector" , "--" , "profile_local" , "eprintln" , env . rustc_stage_2 () . as_str () , "--id" , "Test" , "--cargo" , env . cargo_stage_0 () . as_str () , "--profiles" , profiles . join (",") . as_str () , "--scenarios" , scenarios . join (",") . as_str () , "--exact-match" , crates . join (",") . as_str () ,]) . env ("RUST_LOG" , "collector=debug") . env ("RUSTC" , env . rustc_stage_0 () . as_str ()) . env ("RUSTC_BOOTSTRAP" , "1") . workdir (& env . rustc_perf_dir ()) ; for config in env . benchmark_cargo_config () { cmd = cmd . arg ("--cargo-config") . arg (config) ; } cmd }
};
}
