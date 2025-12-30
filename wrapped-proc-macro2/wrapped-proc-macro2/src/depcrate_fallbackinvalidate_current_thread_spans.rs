// Generated macro for invalidate_current_thread_spans (function)
macro_rules! Depcrate_fallbackinvalidate_current_thread_spans {
() => {
// Module: crate::fallback
// Provides: {"invalidate_current_thread_spans"}
// Dependencies: {}
# [cfg (span_locations)] pub (crate) fn invalidate_current_thread_spans () { # [cfg (not (fuzzing))] SOURCE_MAP . with (| sm | sm . borrow_mut () . files . truncate (1)) ; }
};
}
