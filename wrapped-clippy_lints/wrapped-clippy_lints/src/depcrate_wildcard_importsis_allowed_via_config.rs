// Generated macro for is_allowed_via_config (function)
macro_rules! Depcrate_wildcard_importsis_allowed_via_config {
() => {
// Module: crate::wildcard_imports
// Provides: {"is_allowed_via_config"}
// Dependencies: {}
fn is_allowed_via_config (segments : & [PathSegment < '_ >] , allowed_segments : & FxHashSet < String >) -> bool { segments . iter () . any (| seg | allowed_segments . contains (seg . ident . as_str ())) }
};
}
