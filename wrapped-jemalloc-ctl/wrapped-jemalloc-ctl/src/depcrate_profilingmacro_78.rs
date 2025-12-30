// Generated macro for macro_78 (macro)
macro_rules! Depcrate_profilingmacro_78 {
() => {
// Module: crate::profiling
// Provides: {"macro_78"}
// Dependencies: {}
option ! { prof [str : b"opt.prof\0" , non_str : 2] => bool | ops : r | docs : # [doc = " Memory profiling enabled/disabled."] # [doc = ""] # [doc = " If enabled, profile memory allocation activity."] # [doc = ""] # [doc = " See the `opt.prof_active` option for on-the-fly activation/deactivation."] # [doc = ""] # [doc = " See the `opt.lg_prof_sample` option for probabilistic sampling control."] # [doc = ""] # [doc = " See the `opt.prof_accum` option for control of cumulative sample reporting."] # [doc = ""] # [doc = " See the `opt.lg_prof_interval` option for information on interval-triggered profile"] # [doc = " dumping, the `opt.prof_gdump` option for information on high-water-triggered profile"] # [doc = " dumping, and the `opt.prof_final` option for final profile dumping."] # [doc = ""] # [doc = " Profile output is compatible with the jeprof command, which is based on the pprof that is"] # [doc = " developed as part of the gperftools package. See `HEAP PROFILE FORMAT` for heap profile"] # [doc = " format documentation."] # [doc = ""] # [doc = " # Examples"] # [doc = ""] # [doc = " ```"] # [doc = " # #[global_allocator]"] # [doc = " # static ALLOC: tikv_jemallocator::Jemalloc = tikv_jemallocator::Jemalloc;"] # [doc = " #"] # [doc = " # fn main() {"] # [doc = " use tikv_jemalloc_ctl::profiling;"] # [doc = " let prof = profiling::prof::read().unwrap();"] # [doc = " println!(\"is memory profiling enabled: {}\", prof);"] # [doc = " # }"] # [doc = " ```"] mib_docs : # [doc = " See [`prof`]."] }
};
}
