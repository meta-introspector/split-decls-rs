// Generated macro for OutgoingMessage (enum)
macro_rules! Depcrate_connectionOutgoingMessage {
() => {
// Module: crate::connection
// Provides: {"OutgoingMessage"}
// Dependencies: {}
# [doc = " Enum defining the messages we can send"] # [derive (Debug , Serialize)] pub enum OutgoingMessage < 'a > { BeginningBenchmarkGroup { group : & 'a str , } , FinishedBenchmarkGroup { group : & 'a str , } , BeginningBenchmark { id : RawBenchmarkId , } , SkippingBenchmark { id : RawBenchmarkId , } , Warmup { id : RawBenchmarkId , nanos : f64 , } , MeasurementStart { id : RawBenchmarkId , sample_count : u64 , estimate_ns : f64 , iter_count : u64 , } , MeasurementComplete { id : RawBenchmarkId , iters : & 'a [f64] , times : & 'a [f64] , plot_config : PlotConfiguration , sampling_method : SamplingMethod , benchmark_config : BenchmarkConfig , } , FormattedValue { value : String , } , ScaledValues { scaled_values : Vec < f64 > , unit : & 'a str , } , }
};
}
