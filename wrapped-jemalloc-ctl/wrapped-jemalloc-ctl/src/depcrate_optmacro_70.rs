// Generated macro for macro_70 (macro)
macro_rules! Depcrate_optmacro_70 {
() => {
// Module: crate::opt
// Provides: {"macro_70"}
// Dependencies: {}
option ! { zero [str : b"opt.zero\0" , non_str : 2] => bool | ops : r | docs : # [doc = " `jemalloc`'s zeroing behavior."] # [doc = ""] # [doc = " Requires `--enable-fill` to have been specified during build"] # [doc = " configuration."] # [doc = ""] # [doc = " If enabled, `jemalloc` will initialize each byte of uninitialized"] # [doc = " allocated memory to 0. This is intended for debugging and will impact"] # [doc = " performance negatively. It is disabled by default."] # [doc = ""] # [doc = " # Examples"] # [doc = ""] # [doc = " ```"] # [doc = " # #[global_allocator]"] # [doc = " # static ALLOC: tikv_jemallocator::Jemalloc = tikv_jemallocator::Jemalloc;"] # [doc = " #"] # [doc = " # fn main() {"] # [doc = " use tikv_jemalloc_ctl::opt;"] # [doc = " let zero = opt::zero::read().unwrap();"] # [doc = " println!(\"zeroing: {}\", zero);"] # [doc = " # }"] # [doc = " ```"] mib_docs : # [doc = " See [`zero`]."] }
};
}
