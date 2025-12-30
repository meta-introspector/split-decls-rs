// Generated macro for macro_134 (macro)
macro_rules! Depcratemacro_134 {
() => {
// Module: crate
// Provides: {"macro_134"}
// Dependencies: {}
option ! { max_background_threads [str : b"max_background_threads\0" , non_str : 1] => libc :: size_t | ops : r , w , u | docs : # [doc = " Maximum number of background threads that will be created."] # [doc = ""] # [doc = " ```"] # [doc = " # #[global_allocator]"] # [doc = " # static ALLOC: tikv_jemallocator::Jemalloc = tikv_jemallocator::Jemalloc;"] # [doc = " #"] # [doc = " # fn main() {"] # [doc = " # #[cfg(not(target_os = \"macos\"))] {"] # [doc = " #"] # [doc = " use tikv_jemalloc_ctl::max_background_threads;"] # [doc = " let m = max_background_threads::mib().unwrap();"] # [doc = " println!(\"max_background_threads: {}\", m.read().unwrap());"] # [doc = " m.write(2).unwrap();"] # [doc = " assert_eq!(m.read().unwrap(), 2);"] # [doc = " #"] # [doc = " # } // #[cfg(..)]"] # [doc = " # }"] # [doc = " ```"] mib_docs : # [doc = " See [`max_background_threads`]."] }
};
}
