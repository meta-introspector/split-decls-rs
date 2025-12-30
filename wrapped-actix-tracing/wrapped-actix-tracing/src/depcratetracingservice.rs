// Generated macro for TracingService (struct)
macro_rules! DepcrateTracingService {
() => {
// Module: crate
// Provides: {"TracingService"}
// Dependencies: {}
# [doc = " A `Service` implementation that automatically enters/exits tracing spans"] # [doc = " for the wrapped inner service."] # [derive (Clone)] pub struct TracingService < S , F > { inner : S , make_span : F , }
};
}
