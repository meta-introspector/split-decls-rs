// Generated macro for impl_27 (impl)
macro_rules! Depcrate_configimpl_27 {
() => {
// Module: crate::config
// Provides: {"impl_27"}
// Dependencies: {}
impl RuntimeConfig { pub (crate) fn new (other : impl sealed :: SerializerConfig) -> Self { Self { is_human_readable : other . is_human_readable () , is_named : other . is_named () , bytes : other . bytes () , } } }
};
}
