// Generated macro for impl_703 (impl)
macro_rules! Depcrateimpl_703 {
() => {
// Module: crate
// Provides: {"impl_703"}
// Dependencies: {}
impl SamplingMode { pub (crate) fn choose_sampling_mode (& self , warmup_mean_execution_time : f64 , sample_count : u64 , target_time : f64 ,) -> ActualSamplingMode { match self { SamplingMode :: Linear => ActualSamplingMode :: Linear , SamplingMode :: Flat => ActualSamplingMode :: Flat , SamplingMode :: Auto => { let total_runs = sample_count * (sample_count + 1) / 2 ; let d = (target_time / warmup_mean_execution_time / total_runs as f64) . ceil () as u64 ; let expected_ns = total_runs as f64 * d as f64 * warmup_mean_execution_time ; if expected_ns > (2.0 * target_time) { ActualSamplingMode :: Flat } else { ActualSamplingMode :: Linear } } } } }
};
}
