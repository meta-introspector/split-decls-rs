// Generated macro for impl_231 (impl)
macro_rules! Depcrate_frameimpl_231 {
() => {
// Module: crate::frame
// Provides: {"impl_231"}
// Dependencies: {}
impl H3iFrame { # [doc = " Try to convert this `H3iFrame` to an [EnrichedHeaders]."] # [doc = ""] # [doc = " Returns `Some` if the operation succeeded."] pub fn to_enriched_headers (& self) -> Option < EnrichedHeaders > { if let H3iFrame :: Headers (header) = self { Some (header . clone ()) } else { None } } }
};
}
