macro_rules! BenchmarkFilter {
    () => {
        # [doc = " Benchmark filtering support."] # [derive (Clone , Debug)] pub enum BenchmarkFilter { # [doc = " Run all benchmarks."] AcceptAll , # [doc = " Run benchmarks matching this regex."] Regex (Regex) , # [doc = " Run the benchmark matching this string exactly."] Exact (String) , # [doc = " Do not run any benchmarks."] RejectAll , }
    };
}

BenchmarkFilter!()