// Generated macro for impl_774 (impl)
macro_rules! Depcrate_extensions_apollo_tracingimpl_774 {
() => {
// Module: crate::extensions::apollo_tracing
// Provides: {"impl_774"}
// Dependencies: {}
impl ExtensionFactory for ApolloTracing { fn create (& self) -> Arc < dyn Extension > { Arc :: new (ApolloTracingExtension { inner : Mutex :: new (Inner { start_time : Utc :: now () , end_time : Utc :: now () , resolves : Default :: default () , }) , }) } }
};
}
