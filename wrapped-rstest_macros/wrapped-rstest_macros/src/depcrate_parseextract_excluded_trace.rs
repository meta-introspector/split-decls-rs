// Generated macro for extract_excluded_trace (function)
macro_rules! Depcrate_parseextract_excluded_trace {
() => {
// Module: crate::parse
// Provides: {"extract_excluded_trace"}
// Dependencies: {}
pub (crate) fn extract_excluded_trace (item_fn : & mut ItemFn) -> Result < Vec < Pat > , ErrorsVec > { let mut excluded_trace_extractor = ExcludedTraceAttributesFunctionExtractor :: default () ; excluded_trace_extractor . visit_item_fn_mut (item_fn) ; excluded_trace_extractor . take () }
};
}
