// Generated macro for ModSpans (struct)
macro_rules! Depcrate_astModSpans {
() => {
// Module: crate::ast
// Provides: {"ModSpans"}
// Dependencies: {}
# [derive (Copy , Clone , Encodable , Decodable , Debug , Default , Walkable)] pub struct ModSpans { # [doc = " `inner_span` covers the body of the module; for a file module, its the whole file."] # [doc = " For an inline module, its the span inside the `{ ... }`, not including the curly braces."] pub inner_span : Span , pub inject_use_span : Span , }
};
}
