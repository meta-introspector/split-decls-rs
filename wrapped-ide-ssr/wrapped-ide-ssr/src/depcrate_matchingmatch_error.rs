// Generated macro for match_error (macro)
macro_rules! Depcrate_matchingmatch_error {
() => {
// Module: crate::matching
// Provides: {"match_error"}
// Dependencies: {}
macro_rules ! match_error { ($ e : expr) => { { MatchFailed { reason : if recording_match_fail_reasons () { Some (format ! ("{}" , $ e)) } else { None } } } } ; ($ fmt : expr , $ ($ arg : tt) +) => { { MatchFailed { reason : if recording_match_fail_reasons () { Some (format ! ($ fmt , $ ($ arg) +)) } else { None } } } } ; }
};
}
