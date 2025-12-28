macro_rules! deps {
    () => {
        Profiler!();
        ExternalProfiler!();
    };
}

macro_rules! impl_257 {
    () => {
        deps!();
        impl Profiler for ExternalProfiler { fn start_profiling (& mut self , _benchmark_id : & str , _benchmark_dir : & Path) { } fn stop_profiling (& mut self , _benchmark_id : & str , _benchmark_dir : & Path) { } }
    };
}

impl_257!();