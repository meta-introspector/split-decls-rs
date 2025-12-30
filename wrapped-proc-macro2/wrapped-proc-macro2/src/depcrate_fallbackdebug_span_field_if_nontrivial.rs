// Generated macro for debug_span_field_if_nontrivial (function)
macro_rules! Depcrate_fallbackdebug_span_field_if_nontrivial {
() => {
// Module: crate::fallback
// Provides: {"debug_span_field_if_nontrivial"}
// Dependencies: {}
pub (crate) fn debug_span_field_if_nontrivial (debug : & mut fmt :: DebugStruct , span : Span) { # [cfg (span_locations)] { if span . is_call_site () { return ; } } if cfg ! (span_locations) { debug . field ("span" , & span) ; } }
};
}
