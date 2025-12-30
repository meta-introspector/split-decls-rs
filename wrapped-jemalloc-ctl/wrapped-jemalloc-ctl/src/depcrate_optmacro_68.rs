// Generated macro for macro_68 (macro)
macro_rules! Depcrate_optmacro_68 {
() => {
// Module: crate::opt
// Provides: {"macro_68"}
// Dependencies: {}
option ! { narenas [str : b"opt.narenas\0" , non_str : 2] => libc :: c_uint | ops : r | docs : # [doc = " Maximum number of arenas to use for automatic multiplexing of threads"] # [doc = " and arenas."] # [doc = ""] # [doc = " The default is four times the number of CPUs, or one if there is a"] # [doc = " single CPU."] # [doc = ""] # [doc = " # Examples"] # [doc = ""] # [doc = " ```"] # [doc = " # #[global_allocator]"] # [doc = " # static ALLOC: tikv_jemallocator::Jemalloc = tikv_jemallocator::Jemalloc;"] # [doc = " #"] # [doc = " # fn main() {"] # [doc = " use tikv_jemalloc_ctl::opt;"] # [doc = " let narenas = opt::narenas::read().unwrap();"] # [doc = " println!(\"number of arenas: {}\", narenas);"] # [doc = " # }"] # [doc = " ```"] mib_docs : # [doc = " See [`narenas`]."] }
};
}
