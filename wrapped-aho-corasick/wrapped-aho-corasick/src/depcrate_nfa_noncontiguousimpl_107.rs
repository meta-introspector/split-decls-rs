// Generated macro for impl_107 (impl)
macro_rules! Depcrate_nfa_noncontiguousimpl_107 {
() => {
// Module: crate::nfa::noncontiguous
// Provides: {"impl_107"}
// Dependencies: {}
impl Match { # [doc = " Return the pattern ID for this match."] pub (crate) fn pattern (& self) -> PatternID { self . pid } # [doc = " Return the ID of the next match."] fn link (& self) -> StateID { self . link } }
};
}
