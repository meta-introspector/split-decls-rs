// Generated macro for macro_66 (macro)
macro_rules! Depcrate_optmacro_66 {
() => {
// Module: crate::opt
// Provides: {"macro_66"}
// Dependencies: {}
option ! { abort [str : b"opt.abort\0" , non_str : 2] => bool | ops : r | docs : # [doc = " Whether `jemalloc` calls `abort(3)` on most warnings."] # [doc = ""] # [doc = " This is disabled by default unless `--enable-debug` was specified during"] # [doc = " build configuration."] # [doc = ""] # [doc = " # Examples"] # [doc = ""] # [doc = " ```"] # [doc = " # #[global_allocator]"] # [doc = " # static ALLOC: tikv_jemallocator::Jemalloc = tikv_jemallocator::Jemalloc;"] # [doc = " #"] # [doc = " # fn main() {"] # [doc = " use tikv_jemalloc_ctl::opt;"] # [doc = " let abort = opt::abort::mib().unwrap();"] # [doc = " println!(\"abort on warning: {}\", abort.read().unwrap());"] # [doc = " # }"] # [doc = " ```"] mib_docs : # [doc = " See [`abort`]."] }
};
}
