// Generated macro for impl_443 (impl)
macro_rules! Depcrate_configimpl_443 {
() => {
// Module: crate::config
// Provides: {"impl_443"}
// Dependencies: {}
impl < 'cfg > Binding for ConfigEntry < 'cfg > { type Raw = * mut raw :: git_config_entry ; unsafe fn from_raw (raw : * mut raw :: git_config_entry) -> ConfigEntry < 'cfg > { ConfigEntry { raw , _marker : marker :: PhantomData , owned : true , } } fn raw (& self) -> * mut raw :: git_config_entry { self . raw } }
};
}
