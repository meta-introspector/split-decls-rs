// Generated macro for Transition (struct)
macro_rules! Depcrate_nfa_thompson_nfaTransition {
() => {
// Module: crate::nfa::thompson::nfa
// Provides: {"Transition"}
// Dependencies: {}
# [doc = " A single transition to another state."] # [doc = ""] # [doc = " This transition may only be followed if the current byte in the haystack"] # [doc = " falls in the inclusive range of bytes specified."] # [derive (Clone , Copy , Eq , Hash , PartialEq)] pub struct Transition { # [doc = " The inclusive start of the byte range."] pub start : u8 , # [doc = " The inclusive end of the byte range."] pub end : u8 , # [doc = " The identifier of the state to transition to."] pub next : StateID , }
};
}
