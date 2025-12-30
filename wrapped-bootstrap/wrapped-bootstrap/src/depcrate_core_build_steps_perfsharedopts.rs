// Generated macro for SharedOpts (struct)
macro_rules! Depcrate_core_build_steps_perfSharedOpts {
() => {
// Module: crate::core::build_steps::perf
// Provides: {"SharedOpts"}
// Dependencies: {}
# [derive (Debug , Clone , clap :: Parser)] struct SharedOpts { # [doc = " Select the benchmarks that you want to run (separated by commas)."] # [doc = " If unspecified, all benchmarks will be executed."] # [clap (long , global = true , value_delimiter = ',')] include : Vec < String > , # [doc = " Select the benchmarks matching a prefix in this comma-separated list that you don't want to run."] # [clap (long , global = true , value_delimiter = ',')] exclude : Vec < String > , # [doc = " Select the scenarios that should be benchmarked."] # [clap (long , global = true , value_delimiter = ',' , default_value = "Full,IncrFull,IncrUnchanged,IncrPatched")] scenarios : Vec < Scenario > , # [doc = " Select the profiles that should be benchmarked."] # [clap (long , global = true , value_delimiter = ',' , default_value = "Check,Debug,Opt")] profiles : Vec < Profile > , }
};
}
