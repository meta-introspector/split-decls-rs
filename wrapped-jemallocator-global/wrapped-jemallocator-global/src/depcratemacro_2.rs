// Generated macro for macro_2 (macro)
macro_rules! Depcratemacro_2 {
() => {
// Module: crate
// Provides: {"macro_2"}
// Dependencies: {}
cfg_if ! { if # [cfg (any (feature = "force_global_jemalloc" , target_os = "linux" , target_os = "macos" , target_os = "freebsd" , target_os = "openbsd" , target_os = "netbsd"))] { # [doc = " Sets `jemalloc` as the `#[global_allocator]`."] # [global_allocator] pub static JEMALLOC : tikv_jemallocator :: Jemalloc = tikv_jemallocator :: Jemalloc ; } }
};
}
