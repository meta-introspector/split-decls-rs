// Generated macro for macro_103 (macro)
macro_rules! Depcrate_statsmacro_103 {
() => {
// Module: crate::stats
// Provides: {"macro_103"}
// Dependencies: {}
option ! { active [str : b"stats.active\0" , non_str : 2] => libc :: size_t | ops : r | docs : # [doc = " Total number of bytes in active pages allocated by the application."] # [doc = ""] # [doc = " This is a multiple of the page size, and greater than or equal to the"] # [doc = " value returned by [`allocated`]."] # [doc = ""] # [doc = " This statistic is cached, and is only refreshed when the epoch is"] # [doc = " advanced. See the [`crate::epoch`] type for more information."] # [doc = ""] # [doc = " This corresponds to `stats.active` in jemalloc's API."] # [doc = ""] # [doc = " # Examples"] # [doc = ""] # [doc = " ```rust"] # [doc = " # #[global_allocator]"] # [doc = " # static ALLOC: tikv_jemallocator::Jemalloc = tikv_jemallocator::Jemalloc;"] # [doc = " #"] # [doc = " # fn main() {"] # [doc = " use tikv_jemalloc_ctl::{epoch, stats};"] # [doc = " let e = epoch::mib().unwrap();"] # [doc = " let active = stats::active::mib().unwrap();"] # [doc = ""] # [doc = " let a = active.read().unwrap();"] # [doc = " let _buf = vec![0; 1024 * 1024];"] # [doc = " e.advance().unwrap();"] # [doc = " let b = active.read().unwrap();"] # [doc = " assert!(a < b);"] # [doc = " # }"] # [doc = " ```"] mib_docs : # [doc = " See [`active`]."] }
};
}
