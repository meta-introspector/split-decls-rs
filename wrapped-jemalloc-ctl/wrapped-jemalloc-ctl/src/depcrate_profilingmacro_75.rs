// Generated macro for macro_75 (macro)
macro_rules! Depcrate_profilingmacro_75 {
() => {
// Module: crate::profiling
// Provides: {"macro_75"}
// Dependencies: {}
option ! { lg_prof_interval [str : b"opt.lg_prof_interval\0" , non_str : 2] => libc :: ssize_t | ops : r | docs : # [doc = " Average interval (log base 2) between memory profile dumps, as measured in bytes of"] # [doc = " allocation activity."] # [doc = ""] # [doc = " The actual interval between dumps may be sporadic because"] # [doc = " decentralized allocation counters are used to avoid synchronization bottlenecks."] # [doc = ""] # [doc = " Profiles are dumped to files named according to the pattern"] # [doc = " \\<prefix\\>.\\<pid\\>.\\<seq\\>.i\\<iseq\\>.heap, where \\<prefix\\> is controlled by the"] # [doc = " opt.prof_prefix and prof.prefix options. By default, interval-triggered profile dumping is"] # [doc = " disabled (encoded as -1)."] # [doc = ""] # [doc = " # Examples"] # [doc = ""] # [doc = " ```"] # [doc = " # #[global_allocator]"] # [doc = " # static ALLOC: tikv_jemallocator::Jemalloc = tikv_jemallocator::Jemalloc;"] # [doc = " #"] # [doc = " # fn main() {"] # [doc = " use tikv_jemalloc_ctl::profiling;"] # [doc = " let lg_prof_interval = profiling::lg_prof_interval::read().unwrap();"] # [doc = " println!(\"average interval between memory profile dumps: {}\", lg_prof_interval);"] # [doc = " # }"] # [doc = " ```"] mib_docs : # [doc = " See [`lg_prof_interval`]."] }
};
}
