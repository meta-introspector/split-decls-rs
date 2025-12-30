// Generated macro for macro_69 (macro)
macro_rules! Depcrate_optmacro_69 {
() => {
// Module: crate::opt
// Provides: {"macro_69"}
// Dependencies: {}
option ! { junk [str : b"opt.junk\0" , str : 2] => &'static str | ops : r | docs : # [doc = " `jemalloc`'s junk filling mode."] # [doc = ""] # [doc = " Requires `--enable-fill` to have been specified during build"] # [doc = " configuration."] # [doc = ""] # [doc = " If set to \"alloc\", each byte of uninitialized allocated memory will be"] # [doc = " set to `0x5a`. If set to \"free\", each byte of deallocated memory will be set"] # [doc = " to `0x5a`. If set to \"true\", both allocated and deallocated memory will be"] # [doc = " initialized, and if set to \"false\" junk filling will be disabled. This is"] # [doc = " intended for debugging and will impact performance negatively."] # [doc = ""] # [doc = " The default is \"false\", unless `--enable-debug` was specified during"] # [doc = " build configuration, in"] # [doc = " which case the default is \"true\"."] # [doc = ""] # [doc = " # Examples"] # [doc = ""] # [doc = " ```"] # [doc = " # #[global_allocator]"] # [doc = " # static ALLOC: tikv_jemallocator::Jemalloc = tikv_jemallocator::Jemalloc;"] # [doc = " #"] # [doc = " # fn main() {"] # [doc = " use tikv_jemalloc_ctl::opt;"] # [doc = " let junk = opt::junk::read().unwrap();"] # [doc = " println!(\"junk filling: {}\", junk);"] # [doc = " # }"] # [doc = " ```"] mib_docs : # [doc = " See [`junk`]."] }
};
}
