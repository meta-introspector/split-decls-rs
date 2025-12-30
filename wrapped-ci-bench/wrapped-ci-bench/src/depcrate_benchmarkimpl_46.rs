// Generated macro for impl_46 (impl)
macro_rules! Depcrate_benchmarkimpl_46 {
() => {
// Module: crate::benchmark
// Provides: {"impl_46"}
// Dependencies: {}
impl Benchmark { # [doc = " Create a new benchmark"] pub fn new (name : String , kind : BenchmarkKind , params : BenchmarkParams) -> Self { Self { name , kind , params } } # [doc = " Returns the benchmark's unique name"] pub fn name (& self) -> & str { & self . name } # [doc = " Returns the benchmark's unique name with the side appended to it"] pub fn name_with_side (& self , side : Side) -> String { format ! ("{}_{}" , self . name , side . as_str ()) } }
};
}
