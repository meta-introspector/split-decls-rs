// Generated macro for macro_76 (macro)
macro_rules! Depcrate_profilingmacro_76 {
() => {
// Module: crate::profiling
// Provides: {"macro_76"}
// Dependencies: {}
option ! { lg_prof_sample [str : b"opt.lg_prof_sample\0" , non_str : 2] => libc :: size_t | ops : r | docs : # [doc = " Average interval (log base 2) between allocation samples, as measured in bytes of"] # [doc = " allocation activity. Increasing the sampling interval decreases profile fidelity, but also"] # [doc = " decreases the computational overhead."] # [doc = ""] # [doc = " The default sample interval is 512 KiB (2^19 B)."] # [doc = ""] # [doc = " # Examples"] # [doc = ""] # [doc = " ```"] # [doc = " # #[global_allocator]"] # [doc = " # static ALLOC: tikv_jemallocator::Jemalloc = tikv_jemallocator::Jemalloc;"] # [doc = " #"] # [doc = " # fn main() {"] # [doc = " use tikv_jemalloc_ctl::profiling;"] # [doc = " let lg_prof_sample = profiling::lg_prof_sample::read().unwrap();"] # [doc = " println!(\"average interval between allocation samples: {}\", lg_prof_sample);"] # [doc = " # }"] # [doc = " ```"] mib_docs : # [doc = " See [`lg_prof_sample`]."] }
};
}
