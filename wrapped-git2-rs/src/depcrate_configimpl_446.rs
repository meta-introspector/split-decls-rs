// Generated macro for impl_446 (impl)
macro_rules! Depcrate_configimpl_446 {
() => {
// Module: crate::config
// Provides: {"impl_446"}
// Dependencies: {}
impl < 'cfg > Drop for ConfigEntries < 'cfg > { fn drop (& mut self) { unsafe { raw :: git_config_iterator_free (self . raw) } } }
};
}
