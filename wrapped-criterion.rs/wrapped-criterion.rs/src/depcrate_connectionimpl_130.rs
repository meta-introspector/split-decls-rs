// Generated macro for impl_130 (impl)
macro_rules! Depcrate_connectionimpl_130 {
() => {
// Module: crate::connection
// Provides: {"impl_130"}
// Dependencies: {}
impl From < & crate :: benchmark :: BenchmarkConfig > for BenchmarkConfig { fn from (other : & crate :: benchmark :: BenchmarkConfig) -> Self { BenchmarkConfig { confidence_level : other . confidence_level , measurement_time : other . measurement_time . into () , noise_threshold : other . noise_threshold , nresamples : other . nresamples , sample_size : other . sample_size , significance_level : other . significance_level , warm_up_time : other . warm_up_time . into () , } } }
};
}
