// Generated macro for macro_107 (macro)
macro_rules! Depcrate_statsmacro_107 {
() => {
// Module: crate::stats
// Provides: {"macro_107"}
// Dependencies: {}
option ! { retained [str : b"stats.retained\0" , non_str : 2] => libc :: size_t | ops : r | docs : # [doc = " Total number of bytes in virtual memory mappings that were retained"] # [doc = " rather than being returned to the operating system via e.g. `munmap(2)`."] # [doc = ""] # [doc = " Retained virtual memory is typically untouched, decommitted, or purged,"] # [doc = " so it has no strongly associated physical memory. Retained memory is"] # [doc = " excluded from mapped memory statistics, e.g. [`mapped`]."] # [doc = ""] # [doc = " This statistic is cached, and is only refreshed when the epoch is"] # [doc = " advanced. See the [`crate::epoch`] type for more information."] # [doc = ""] # [doc = " This corresponds to `stats.retained` in jemalloc's API."] # [doc = ""] # [doc = " # Examples"] # [doc = ""] # [doc = " ```rust"] # [doc = " # #[global_allocator]"] # [doc = " # static ALLOC: tikv_jemallocator::Jemalloc = tikv_jemallocator::Jemalloc;"] # [doc = " #"] # [doc = " # fn main() {"] # [doc = " use tikv_jemalloc_ctl::{epoch, stats};"] # [doc = " let e = epoch::mib().unwrap();"] # [doc = " let retained = stats::retained::mib().unwrap();"] # [doc = ""] # [doc = " e.advance().unwrap();"] # [doc = " let size = retained.read().unwrap();"] # [doc = " println!(\"{} bytes of total retained data\", size);"] # [doc = " # }"] # [doc = " ```"] mib_docs : # [doc = " See [`retained`]."] }
};
}
