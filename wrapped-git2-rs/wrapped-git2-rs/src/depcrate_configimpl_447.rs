// Generated macro for impl_447 (impl)
macro_rules! Depcrate_configimpl_447 {
() => {
// Module: crate::config
// Provides: {"impl_447"}
// Dependencies: {}
impl < 'cfg > Drop for ConfigEntry < 'cfg > { fn drop (& mut self) { if self . owned { unsafe { raw :: git_config_entry_free (self . raw) } } } }
};
}
