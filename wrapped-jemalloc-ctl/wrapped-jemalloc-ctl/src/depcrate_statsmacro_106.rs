// Generated macro for macro_106 (macro)
macro_rules! Depcrate_statsmacro_106 {
() => {
// Module: crate::stats
// Provides: {"macro_106"}
// Dependencies: {}
option ! { mapped [str : b"stats.mapped\0" , non_str : 2] => libc :: size_t | ops : r | docs : # [doc = " Total number of bytes in active extents mapped by the allocator."] # [doc = ""] # [doc = " This does not include inactive extents, even those that contain unused"] # [doc = " dirty pages, so there is no strict ordering between this and the value"] # [doc = " returned by [`resident`]. This is a multiple of the page size, and is"] # [doc = " larger than the value returned by [`active`]."] # [doc = ""] # [doc = " This statistic is cached, and is only refreshed when the epoch is"] # [doc = " advanced. See the [`crate::epoch`] type for more information."] # [doc = ""] # [doc = " This corresponds to `stats.mapped` in jemalloc's API."] # [doc = ""] # [doc = " # Examples"] # [doc = ""] # [doc = " ```rust"] # [doc = " # #[global_allocator]"] # [doc = " # static ALLOC: tikv_jemallocator::Jemalloc = tikv_jemallocator::Jemalloc;"] # [doc = " #"] # [doc = " # fn main() {"] # [doc = " use tikv_jemalloc_ctl::{epoch, stats};"] # [doc = " let e = epoch::mib().unwrap();"] # [doc = " let mapped = stats::mapped::mib().unwrap();"] # [doc = ""] # [doc = " e.advance().unwrap();"] # [doc = " let size = mapped.read().unwrap();"] # [doc = " println!(\"{} bytes of total mapped data\", size);"] # [doc = " # }"] # [doc = " ```"] mib_docs : # [doc = " See [`mapped`]."] }
};
}
