// Generated macro for default_default_config (function)
macro_rules! Depcrate_test_runner_configdefault_default_config {
() => {
// Module: crate::test_runner::config
// Provides: {"default_default_config"}
// Dependencies: {}
fn default_default_config () -> Config { Config { cases : 256 , max_local_rejects : 65_536 , max_global_rejects : 1024 , max_flat_map_regens : 1_000_000 , failure_persistence : None , source_file : None , test_name : None , # [cfg (feature = "fork")] fork : false , # [cfg (feature = "timeout")] timeout : 0 , # [cfg (feature = "std")] max_shrink_time : 0 , max_shrink_iters : u32 :: MAX , max_default_size_range : 100 , result_cache : noop_result_cache , # [cfg (feature = "std")] verbose : 0 , rng_algorithm : RngAlgorithm :: default () , rng_seed : RngSeed :: Random , _non_exhaustive : () , } }
};
}
