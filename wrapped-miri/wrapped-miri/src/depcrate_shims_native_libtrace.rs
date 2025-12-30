// Generated macro for trace (module)
macro_rules! Depcrate_shims_native_libtrace {
() => {
// Module: crate::shims::native_lib
// Provides: {"trace"}
// Dependencies: {}
# [cfg_attr (not (all (target_os = "linux" , target_env = "gnu" , any (target_arch = "x86" , target_arch = "x86_64"))) , path = "trace/stub.rs")] pub mod trace ;
};
}
