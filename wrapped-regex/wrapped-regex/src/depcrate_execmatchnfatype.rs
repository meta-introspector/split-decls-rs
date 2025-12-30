// Generated macro for MatchNfaType (enum)
macro_rules! Depcrate_execMatchNfaType {
() => {
// Module: crate::exec
// Provides: {"MatchNfaType"}
// Dependencies: {}
# [derive (Clone , Copy , Debug)] enum MatchNfaType { # [doc = " Choose between Backtrack and PikeVM."] Auto , # [doc = " NFA bounded backtracking."] # [doc = ""] # [doc = " (This is only set by tests, since it never makes sense to always want"] # [doc = " backtracking.)"] Backtrack , # [doc = " The Pike VM."] # [doc = ""] # [doc = " (This is only set by tests, since it never makes sense to always want"] # [doc = " the Pike VM.)"] PikeVM , }
};
}
