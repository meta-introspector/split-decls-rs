// Generated macro for macro_105 (macro)
macro_rules! Depcrate_statsmacro_105 {
() => {
// Module: crate::stats
// Provides: {"macro_105"}
// Dependencies: {}
option ! { resident [str : b"stats.resident\0" , non_str : 2] => libc :: size_t | ops : r | docs : # [doc = " Total number of bytes in physically resident data pages mapped by the"] # [doc = " allocator."] # [doc = ""] # [doc = " This consists of all pages dedicated to allocator metadata, pages"] # [doc = " backing active allocations, and unused dirty pages. It may overestimate"] # [doc = " the true value because pages may not actually be physically resident if"] # [doc = " they correspond to demand-zeroed virtual memory that has not yet been"] # [doc = " touched. This is a multiple of the page size, and is larger than the"] # [doc = " value returned by [`active`]."] # [doc = ""] # [doc = " This statistic is cached, and is only refreshed when the epoch is"] # [doc = " advanced. See the [`crate::epoch`] type for more information."] # [doc = ""] # [doc = " This corresponds to `stats.resident` in jemalloc's API."] # [doc = ""] # [doc = " # Examples"] # [doc = ""] # [doc = " ```rust"] # [doc = " # #[global_allocator]"] # [doc = " # static ALLOC: tikv_jemallocator::Jemalloc = tikv_jemallocator::Jemalloc;"] # [doc = " #"] # [doc = " # fn main() {"] # [doc = " use tikv_jemalloc_ctl::{epoch, stats};"] # [doc = " let e = epoch::mib().unwrap();"] # [doc = " let resident = stats::resident::mib().unwrap();"] # [doc = ""] # [doc = " e.advance().unwrap();"] # [doc = " let size = resident.read().unwrap();"] # [doc = " println!(\"{} bytes of total resident data\", size);"] # [doc = " # }"] # [doc = " ```"] mib_docs : # [doc = " See [`resident`]."] }
};
}
