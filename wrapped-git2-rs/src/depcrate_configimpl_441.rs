// Generated macro for impl_441 (impl)
macro_rules! Depcrate_configimpl_441 {
() => {
// Module: crate::config
// Provides: {"impl_441"}
// Dependencies: {}
impl Drop for Config { fn drop (& mut self) { unsafe { raw :: git_config_free (self . raw) } } }
};
}
