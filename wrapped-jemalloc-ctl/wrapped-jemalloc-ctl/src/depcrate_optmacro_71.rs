// Generated macro for macro_71 (macro)
macro_rules! Depcrate_optmacro_71 {
() => {
// Module: crate::opt
// Provides: {"macro_71"}
// Dependencies: {}
option ! { tcache [str : b"opt.tcache\0" , non_str : 2] => bool | ops : r | docs : # [doc = " Thread-local allocation caching behavior."] # [doc = ""] # [doc = " Thread-specific caching allows many allocations to be satisfied without"] # [doc = " performing any thread synchronization, at the cost of increased memory"] # [doc = " use. This is enabled by default."] # [doc = ""] # [doc = " # Examples"] # [doc = ""] # [doc = " ```"] # [doc = " # #[global_allocator]"] # [doc = " # static ALLOC: tikv_jemallocator::Jemalloc = tikv_jemallocator::Jemalloc;"] # [doc = " #"] # [doc = " # fn main() {"] # [doc = " use tikv_jemalloc_ctl::opt;"] # [doc = " let tcache = opt::tcache::read().unwrap();"] # [doc = " println!(\"thread-local caching: {}\", tcache);"] # [doc = " # }"] # [doc = " ```"] mib_docs : # [doc = " See [`tcache`]."] }
};
}
