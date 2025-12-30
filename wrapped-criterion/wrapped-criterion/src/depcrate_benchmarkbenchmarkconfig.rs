// Generated macro for BenchmarkConfig (struct)
macro_rules! Depcrate_benchmarkBenchmarkConfig {
() => {
// Module: crate::benchmark
// Provides: {"BenchmarkConfig"}
// Dependencies: {}
# [doc = " Struct containing all of the configuration options for a benchmark."] pub struct BenchmarkConfig { pub confidence_level : f64 , pub measurement_time : Duration , pub noise_threshold : f64 , pub nresamples : usize , pub sample_size : usize , pub significance_level : f64 , pub warm_up_time : Duration , pub sampling_mode : SamplingMode , pub quick_mode : bool , }
};
}
