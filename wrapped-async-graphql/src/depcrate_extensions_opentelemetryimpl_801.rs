// Generated macro for impl_801 (impl)
macro_rules! Depcrate_extensions_opentelemetryimpl_801 {
() => {
// Module: crate::extensions::opentelemetry
// Provides: {"impl_801"}
// Dependencies: {}
impl < T > ExtensionFactory for OpenTelemetry < T > where T : Tracer + Send + Sync + 'static , < T as Tracer > :: Span : Sync + Send , { fn create (& self) -> Arc < dyn Extension > { Arc :: new (OpenTelemetryExtension { tracer : self . tracer . clone () , }) } }
};
}
