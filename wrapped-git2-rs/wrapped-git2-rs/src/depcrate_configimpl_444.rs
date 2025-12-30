// Generated macro for impl_444 (impl)
macro_rules! Depcrate_configimpl_444 {
() => {
// Module: crate::config
// Provides: {"impl_444"}
// Dependencies: {}
impl < 'cfg > Binding for ConfigEntries < 'cfg > { type Raw = * mut raw :: git_config_iterator ; unsafe fn from_raw (raw : * mut raw :: git_config_iterator) -> ConfigEntries < 'cfg > { ConfigEntries { raw , current : None , _marker : marker :: PhantomData , } } fn raw (& self) -> * mut raw :: git_config_iterator { self . raw } }
};
}
