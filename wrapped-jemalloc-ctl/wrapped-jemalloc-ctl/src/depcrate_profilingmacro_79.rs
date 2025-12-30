// Generated macro for macro_79 (macro)
macro_rules! Depcrate_profilingmacro_79 {
() => {
// Module: crate::profiling
// Provides: {"macro_79"}
// Dependencies: {}
option ! { prof_leak [str : b"opt.prof_leak\0" , non_str : 2] => bool | ops : r | docs : # [doc = " Leak reporting enabled/disabled."] # [doc = ""] # [doc = " If enabled, use an `atexit(3)` function to report memory leaks detected by allocation"] # [doc = " sampling."] # [doc = ""] # [doc = " See the opt.prof option for information on analyzing heap profile output."] # [doc = ""] # [doc = " Works only when combined with `opt.prof_final`, otherwise does nothing."] # [doc = ""] # [doc = " This option is disabled by default."] # [doc = ""] # [doc = " # Examples"] # [doc = ""] # [doc = " ```"] # [doc = " # #[global_allocator]"] # [doc = " # static ALLOC: tikv_jemallocator::Jemalloc = tikv_jemallocator::Jemalloc;"] # [doc = " #"] # [doc = " # fn main() {"] # [doc = " use tikv_jemalloc_ctl::profiling;"] # [doc = " let prof_leak = profiling::prof_leak::read().unwrap();"] # [doc = " println!(\"is leak reporting enabled: {}\", prof_leak);"] # [doc = " # }"] # [doc = " ```"] mib_docs : # [doc = " See [`prof_leak`]."] }
};
}
