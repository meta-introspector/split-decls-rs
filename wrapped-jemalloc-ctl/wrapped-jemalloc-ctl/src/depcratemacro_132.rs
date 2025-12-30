// Generated macro for macro_132 (macro)
macro_rules! Depcratemacro_132 {
() => {
// Module: crate
// Provides: {"macro_132"}
// Dependencies: {}
option ! { version [str : b"version\0" , str : 1] => &'static str | ops : r | docs : # [doc = " `jemalloc` version string."] # [doc = ""] # [doc = " # Example"] # [doc = ""] # [doc = " ```"] # [doc = " # #[global_allocator]"] # [doc = " # static ALLOC: tikv_jemallocator::Jemalloc = tikv_jemallocator::Jemalloc;"] # [doc = " #"] # [doc = " # fn main() {"] # [doc = " use tikv_jemalloc_ctl::version;"] # [doc = " println!(\"jemalloc version {}\", version::read().unwrap());"] # [doc = " let version_mib = version::mib().unwrap();"] # [doc = " println!(\"jemalloc version {}\", version_mib.read().unwrap());"] # [doc = " # }"] # [doc = " ```"] mib_docs : # [doc = " See [`version`]."] }
};
}
