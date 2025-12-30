// Generated macro for macro_12 (macro)
macro_rules! Depcrate_arenasmacro_12 {
() => {
// Module: crate::arenas
// Provides: {"macro_12"}
// Dependencies: {}
option ! { narenas [str : b"arenas.narenas\0" , non_str : 2] => libc :: c_uint | ops : r | docs : # [doc = " Current limit on the number of arenas."] # [doc = ""] # [doc = " # Examples"] # [doc = ""] # [doc = " ```"] # [doc = " #"] # [doc = " # #[global_allocator]"] # [doc = " # static ALLOC: tikv_jemallocator::Jemalloc = tikv_jemallocator::Jemalloc;"] # [doc = " #"] # [doc = " # fn main() {"] # [doc = " use tikv_jemalloc_ctl::arenas;"] # [doc = " println!(\"number of arenas: {}\", arenas::narenas::read().unwrap());"] # [doc = ""] # [doc = " let arenas_mib = arenas::narenas::mib().unwrap();"] # [doc = " println!(\"number of arenas: {}\", arenas_mib.read().unwrap());"] # [doc = " # }"] # [doc = " ```"] mib_docs : # [doc = " See [`narenas`]."] }
};
}
