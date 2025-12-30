// Generated macro for Criterion (struct)
macro_rules! DepcrateCriterion {
() => {
// Module: crate
// Provides: {"Criterion"}
// Dependencies: {}
# [doc = " The benchmark manager"] # [doc = ""] # [doc = " `Criterion` lets you configure and execute benchmarks"] # [doc = ""] # [doc = " Each benchmark consists of four phases:"] # [doc = ""] # [doc = " - **Warm-up**: The routine is repeatedly executed, to let the CPU/OS/JIT/interpreter adapt to"] # [doc = "   the new load"] # [doc = " - **Measurement**: The routine is repeatedly executed, and timing information is collected into"] # [doc = "   a sample"] # [doc = " - **Analysis**: The sample is analyzed and distilled into meaningful statistics that get"] # [doc = "   reported to stdout, stored in files, and plotted"] # [doc = " - **Comparison**: The current sample is compared with the sample obtained in the previous"] # [doc = "   benchmark."] pub struct Criterion < M : Measurement = WallTime > { config : BenchmarkConfig , filter : BenchmarkFilter , report : Reports , output_directory : PathBuf , baseline_directory : String , baseline : Baseline , load_baseline : Option < String > , all_directories : HashSet < String > , all_titles : HashSet < String > , measurement : M , profiler : Box < RefCell < dyn Profiler > > , connection : Option < MutexGuard < 'static , Connection > > , mode : Mode , }
};
}
