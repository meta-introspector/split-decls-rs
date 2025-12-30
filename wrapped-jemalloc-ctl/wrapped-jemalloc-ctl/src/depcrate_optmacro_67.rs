// Generated macro for macro_67 (macro)
macro_rules! Depcrate_optmacro_67 {
() => {
// Module: crate::opt
// Provides: {"macro_67"}
// Dependencies: {}
option ! { dss [str : b"opt.dss\0" , str : 2] => &'static str | ops : r | docs : # [doc = " The `dss` (`sbrk(2)`) allocation precedence as related to `mmap(2)`"] # [doc = " allocation."] # [doc = ""] # [doc = " The following settings are supported if `sbrk(2)` is supported by the"] # [doc = " operating system: \"disabled\", \"primary\", and \"secondary\"; otherwise only"] # [doc = " \"disabled\" is supported. The default is \"secondary\" if `sbrk(2)` is"] # [doc = " supported by the operating system; \"disabled\" otherwise."] # [doc = ""] # [doc = " # Examples"] # [doc = ""] # [doc = " ```"] # [doc = " # #[global_allocator]"] # [doc = " # static ALLOC: tikv_jemallocator::Jemalloc = tikv_jemallocator::Jemalloc;"] # [doc = " #"] # [doc = " # fn main() {"] # [doc = " use tikv_jemalloc_ctl::opt;"] # [doc = " let dss = opt::dss::read().unwrap();"] # [doc = " println!(\"dss priority: {}\", dss);"] # [doc = " # }"] # [doc = " ```"] mib_docs : # [doc = " See [`dss`]."] }
};
}
