// Generated macro for macro_73 (macro)
macro_rules! Depcrate_optmacro_73 {
() => {
// Module: crate::opt
// Provides: {"macro_73"}
// Dependencies: {}
option ! { background_thread [str : b"opt.background_thread\0" , non_str : 2] => bool | ops : r | docs : # [doc = " `jemalloc`'s default initialization behavior for background threads."] # [doc = ""] # [doc = " `jemalloc` automatically spawns background worker threads on"] # [doc = " initialization (first `jemalloc` call) if this option is enabled. By"] # [doc = " default this option is disabled - `malloc_conf=background_thread:true`"] # [doc = " changes its default."] # [doc = ""] # [doc = " # Examples"] # [doc = ""] # [doc = " ```"] # [doc = " # #[global_allocator]"] # [doc = " # static ALLOC: tikv_jemallocator::Jemalloc = tikv_jemallocator::Jemalloc;"] # [doc = " #"] # [doc = " # fn main() {"] # [doc = " use tikv_jemalloc_ctl::opt;"] # [doc = " let background_thread = opt::background_thread::read().unwrap();"] # [doc = " println!(\"background threads since initialization: {}\", background_thread);"] # [doc = " # }"] # [doc = " ```"] mib_docs : # [doc = " See [`background_thread`]."] }
};
}
