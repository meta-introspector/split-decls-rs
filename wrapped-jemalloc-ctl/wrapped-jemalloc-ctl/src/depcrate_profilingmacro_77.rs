// Generated macro for macro_77 (macro)
macro_rules! Depcrate_profilingmacro_77 {
() => {
// Module: crate::profiling
// Provides: {"macro_77"}
// Dependencies: {}
option ! { prof_final [str : b"opt.prof_final\0" , non_str : 2] => bool | ops : r | docs : # [doc = " Use an atexit(3) function to dump final memory usage to a file named according to the"] # [doc = " pattern \\<prefix\\>.\\<pid\\>.\\<seq\\>.f.heap, where \\<prefix\\> is controlled by the opt.prof_prefix"] # [doc = " and prof.prefix options."] # [doc = ""] # [doc = " Note that atexit() may allocate memory during application initialization and then deadlock"] # [doc = " internally when jemalloc in turn calls `atexit()`, so this option is not universally usable"] # [doc = " (though the application can register its own `atexit()` function with equivalent"] # [doc = " functionality)."] # [doc = ""] # [doc = " This option is disabled by default."] # [doc = ""] # [doc = " # Examples"] # [doc = ""] # [doc = " ```"] # [doc = " # #[global_allocator]"] # [doc = " # static ALLOC: tikv_jemallocator::Jemalloc = tikv_jemallocator::Jemalloc;"] # [doc = " #"] # [doc = " # fn main() {"] # [doc = " use tikv_jemalloc_ctl::profiling;"] # [doc = " let prof_final = profiling::prof_final::read().unwrap();"] # [doc = " println!(\"dump final memory usage to file: {}\", prof_final);"] # [doc = " # }"] # [doc = " ```"] mib_docs : # [doc = " See [`prof_final`]."] }
};
}
