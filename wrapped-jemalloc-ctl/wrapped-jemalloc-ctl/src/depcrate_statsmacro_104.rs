// Generated macro for macro_104 (macro)
macro_rules! Depcrate_statsmacro_104 {
() => {
// Module: crate::stats
// Provides: {"macro_104"}
// Dependencies: {}
option ! { metadata [str : b"stats.metadata\0" , non_str : 2] => libc :: size_t | ops : r | docs : # [doc = " Total number of bytes dedicated to `jemalloc` metadata."] # [doc = ""] # [doc = " This statistic is cached, and is only refreshed when the epoch is"] # [doc = " advanced. See the [`crate::epoch`] type for more information."] # [doc = ""] # [doc = " This corresponds to `stats.metadata` in jemalloc's API."] # [doc = ""] # [doc = " # Examples"] # [doc = ""] # [doc = " ```rust"] # [doc = " # #[global_allocator]"] # [doc = " # static ALLOC: tikv_jemallocator::Jemalloc = tikv_jemallocator::Jemalloc;"] # [doc = " #"] # [doc = " # fn main() {"] # [doc = " use tikv_jemalloc_ctl::{epoch, stats};"] # [doc = " let e = epoch::mib().unwrap();"] # [doc = " let metadata = stats::metadata::mib().unwrap();"] # [doc = ""] # [doc = " e.advance().unwrap();"] # [doc = " let size = metadata.read().unwrap();"] # [doc = " println!(\"{} bytes of jemalloc metadata\", size);"] # [doc = " # }"] # [doc = " ```"] mib_docs : # [doc = " See [`metadata`]."] }
};
}
