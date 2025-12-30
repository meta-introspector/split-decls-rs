// Generated macro for macro_135 (macro)
macro_rules! Depcratemacro_135 {
() => {
// Module: crate
// Provides: {"macro_135"}
// Dependencies: {}
option ! { epoch [str : b"epoch\0" , non_str : 1] => u64 | ops : r , w , u | docs : # [doc = " `jemalloc` epoch."] # [doc = ""] # [doc = " Many of the statistics tracked by `jemalloc` are cached. The epoch"] # [doc = " controls when they are refreshed."] # [doc = ""] # [doc = " # Example"] # [doc = ""] # [doc = " Advancing the epoch:"] # [doc = ""] # [doc = " ```"] # [doc = " # #[global_allocator]"] # [doc = " # static ALLOC: tikv_jemallocator::Jemalloc = tikv_jemallocator::Jemalloc;"] # [doc = " #"] # [doc = " # fn main() {"] # [doc = " #"] # [doc = " use tikv_jemalloc_ctl::epoch;"] # [doc = " let e = epoch::mib().unwrap();"] # [doc = " let a = e.advance().unwrap();"] # [doc = " let b = e.advance().unwrap();"] # [doc = " assert_eq!(a + 1, b);"] # [doc = ""] # [doc = " let o = e.update(0).unwrap();"] # [doc = " assert_eq!(o, e.read().unwrap());"] # [doc = " # }"] mib_docs : # [doc = " See [`epoch`]."] }
};
}
