// Generated macro for record_match_fails_reasons_scope (function)
macro_rules! Depcrate_matchingrecord_match_fails_reasons_scope {
() => {
// Module: crate::matching
// Provides: {"record_match_fails_reasons_scope"}
// Dependencies: {}
pub (crate) fn record_match_fails_reasons_scope < F , T > (debug_active : bool , f : F) -> T where F : Fn () -> T , { RECORDING_MATCH_FAIL_REASONS . with (| c | c . set (debug_active)) ; let res = f () ; RECORDING_MATCH_FAIL_REASONS . with (| c | c . set (false)) ; res }
};
}
