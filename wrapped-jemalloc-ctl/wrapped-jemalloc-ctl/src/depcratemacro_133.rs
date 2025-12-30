// Generated macro for macro_133 (macro)
macro_rules! Depcratemacro_133 {
() => {
// Module: crate
// Provides: {"macro_133"}
// Dependencies: {}
option ! { background_thread [str : b"background_thread\0" , non_str : 1] => bool | ops : r , w , u | docs : # [doc = " State of internal background worker threads."] # [doc = ""] # [doc = " When enabled, background threads are created on demand (the number of"] # [doc = " background threads will be no more than the number of CPUs or active"] # [doc = " arenas). Threads run periodically and handle purging asynchronously."] # [doc = ""] # [doc = " ```"] # [doc = " # #[global_allocator]"] # [doc = " # static ALLOC: tikv_jemallocator::Jemalloc = tikv_jemallocator::Jemalloc;"] # [doc = " #"] # [doc = " # fn main() {"] # [doc = " # #[cfg(not(target_os = \"macos\"))] {"] # [doc = " #"] # [doc = " use tikv_jemalloc_ctl::background_thread;"] # [doc = " let bg = background_thread::mib().unwrap();"] # [doc = " let s = bg.read().unwrap();"] # [doc = " println!(\"background_threads enabled: {}\", s);"] # [doc = " let p = background_thread::update(!s).unwrap();"] # [doc = " println!(\"background_threads enabled: {} => {}\", p, bg.read().unwrap());"] # [doc = " assert_eq!(p, s);"] # [doc = " background_thread::write(s).unwrap();"] # [doc = " println!(\"background_threads enabled: {}\", bg.read().unwrap());"] # [doc = " assert_eq!(p, s);"] # [doc = " #"] # [doc = " # } // #[cfg(..)]"] # [doc = " # }"] # [doc = " ```"] mib_docs : # [doc = " See [`background_thread`]."] }
};
}
