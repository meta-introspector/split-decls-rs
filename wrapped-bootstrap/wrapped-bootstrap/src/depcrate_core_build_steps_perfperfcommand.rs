// Generated macro for PerfCommand (enum)
macro_rules! Depcrate_core_build_steps_perfPerfCommand {
() => {
// Module: crate::core::build_steps::perf
// Provides: {"PerfCommand"}
// Dependencies: {}
# [derive (Debug , Clone , clap :: Parser)] enum PerfCommand { # [doc = " Run `profile_local eprintln`."] # [doc = " This executes the compiler on the given benchmarks and stores its stderr output."] Eprintln { # [clap (flatten)] opts : SharedOpts , } , # [doc = " Run `profile_local samply`"] # [doc = " This executes the compiler on the given benchmarks and profiles it with `samply`."] # [doc = " You need to install `samply`, e.g. using `cargo install samply`."] Samply { # [clap (flatten)] opts : SharedOpts , } , # [doc = " Run `profile_local cachegrind`."] # [doc = " This executes the compiler on the given benchmarks under `Cachegrind`."] Cachegrind { # [clap (flatten)] opts : SharedOpts , } , # [doc = " Run compile benchmarks with a locally built compiler."] Benchmark { # [doc = " Identifier to associate benchmark results with"] # [clap (name = "benchmark-id")] id : String , # [clap (flatten)] opts : SharedOpts , } , # [doc = " Compare the results of two previously executed benchmark runs."] Compare { # [doc = " The name of the base artifact to be compared."] base : String , # [doc = " The name of the modified artifact to be compared."] modified : String , } , }
};
}
