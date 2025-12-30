// Generated macro for macro_102 (macro)
macro_rules! Depcrate_statsmacro_102 {
() => {
// Module: crate::stats
// Provides: {"macro_102"}
// Dependencies: {}
option ! { allocated [str : b"stats.allocated\0" , non_str : 2] => libc :: size_t | ops : r | docs : # [doc = " Total number of bytes allocated by the application."] # [doc = ""] # [doc = " This statistic is cached, and is only refreshed when the epoch is"] # [doc = " advanced. See the [`crate::epoch`] type for more information."] # [doc = ""] # [doc = " This corresponds to `stats.allocated` in jemalloc's API."] # [doc = ""] # [doc = " # Examples"] # [doc = ""] # [doc = " ```rust"] # [doc = " # #[global_allocator]"] # [doc = " # static ALLOC: tikv_jemallocator::Jemalloc = tikv_jemallocator::Jemalloc;"] # [doc = " #"] # [doc = " # fn main() {"] # [doc = " use tikv_jemalloc_ctl::{epoch, stats};"] # [doc = " let e = epoch::mib().unwrap();"] # [doc = " let allocated = stats::allocated::mib().unwrap();"] # [doc = ""] # [doc = " let a = allocated.read().unwrap();"] # [doc = " let _buf = vec![0; 1024 * 1024];"] # [doc = " e.advance().unwrap();"] # [doc = " let b = allocated.read().unwrap();"] # [doc = " assert!(a < b);"] # [doc = " # }"] # [doc = " ```"] mib_docs : # [doc = " See [`allocated`]."] }
};
}
