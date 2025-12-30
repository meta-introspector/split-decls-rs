// Generated macro for macro_72 (macro)
macro_rules! Depcrate_optmacro_72 {
() => {
// Module: crate::opt
// Provides: {"macro_72"}
// Dependencies: {}
option ! { tcache_max [str : b"opt.tcache_max\0" , non_str : 2] => libc :: size_t | ops : r | docs : # [doc = " Maximum size class (log base 2) to cache in the thread-specific cache"] # [doc = " (`tcache`)."] # [doc = ""] # [doc = " At a minimum, all small size classes are cached, and at a maximum all"] # [doc = " large size classes are cached. The default maximum is 32 KiB (2^15)."] # [doc = ""] # [doc = " # Examples"] # [doc = ""] # [doc = " ```"] # [doc = " # #[global_allocator]"] # [doc = " # static ALLOC: tikv_jemallocator::Jemalloc = tikv_jemallocator::Jemalloc;"] # [doc = " #"] # [doc = " # fn main() {"] # [doc = " use tikv_jemalloc_ctl::opt;"] # [doc = " let tcache_max = opt::tcache_max::read().unwrap();"] # [doc = " println!(\"max cached allocation size: {}\", tcache_max);"] # [doc = " # }"] # [doc = " ```"] mib_docs : # [doc = " See [`tcache_max`]."] }
};
}
