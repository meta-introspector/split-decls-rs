// Generated macro for impl_811 (impl)
macro_rules! Depcrate_extensions_tracingimpl_811 {
() => {
// Module: crate::extensions::tracing
// Provides: {"impl_811"}
// Dependencies: {}
impl ExtensionFactory for Tracing { fn create (& self) -> Arc < dyn Extension > { Arc :: new (TracingExtension) } }
};
}
