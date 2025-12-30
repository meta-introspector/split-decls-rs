// Generated macro for impl_800 (impl)
macro_rules! Depcrate_extensions_opentelemetryimpl_800 {
() => {
// Module: crate::extensions::opentelemetry
// Provides: {"impl_800"}
// Dependencies: {}
impl < T > OpenTelemetry < T > { # [doc = " Use `tracer` to create an OpenTelemetry extension."] pub fn new (tracer : T) -> OpenTelemetry < T > where T : Tracer + Send + Sync + 'static , < T as Tracer > :: Span : Sync + Send , { Self { tracer : Arc :: new (tracer) , } } }
};
}
