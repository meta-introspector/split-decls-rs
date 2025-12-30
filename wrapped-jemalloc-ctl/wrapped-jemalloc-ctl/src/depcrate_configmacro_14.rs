// Generated macro for macro_14 (macro)
macro_rules! Depcrate_configmacro_14 {
() => {
// Module: crate::config
// Provides: {"macro_14"}
// Dependencies: {}
option ! { malloc_conf [str : b"config.malloc_conf\0" , str : 2] => &'static str | ops : r | docs : # [doc = " Default run-time options specified during `jemalloc`'s build configuration."] # [doc = ""] # [doc = " The string will be empty unless `--with-malloc-conf` was specified"] # [doc = " during build configuration."] # [doc = ""] # [doc = " # Examples"] # [doc = ""] # [doc = " ```"] # [doc = " # #[global_allocator]"] # [doc = " # static ALLOC: tikv_jemallocator::Jemalloc = tikv_jemallocator::Jemalloc;"] # [doc = " #"] # [doc = " # fn main() {"] # [doc = " use tikv_jemalloc_ctl::config;"] # [doc = " let malloc_conf = config::malloc_conf::mib().unwrap();"] # [doc = " println!(\"default malloc conf: {}\", malloc_conf.read().unwrap());"] # [doc = " # }"] # [doc = " ```"] mib_docs : # [doc = " See [`malloc_conf`]."] }
};
}
