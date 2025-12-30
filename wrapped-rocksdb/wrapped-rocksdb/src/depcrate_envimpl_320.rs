// Generated macro for impl_320 (impl)
macro_rules! Depcrate_envimpl_320 {
() => {
// Module: crate::env
// Provides: {"impl_320"}
// Dependencies: {}
impl Drop for EnvWrapper { fn drop (& mut self) { unsafe { ffi :: rocksdb_env_destroy (self . inner) ; } } }
};
}
