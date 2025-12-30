// Generated macro for impl_440 (impl)
macro_rules! Depcrate_configimpl_440 {
() => {
// Module: crate::config
// Provides: {"impl_440"}
// Dependencies: {}
impl Binding for Config { type Raw = * mut raw :: git_config ; unsafe fn from_raw (raw : * mut raw :: git_config) -> Config { Config { raw } } fn raw (& self) -> * mut raw :: git_config { self . raw } }
};
}
