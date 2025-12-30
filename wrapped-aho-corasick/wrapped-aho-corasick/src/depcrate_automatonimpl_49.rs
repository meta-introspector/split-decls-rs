// Generated macro for impl_49 (impl)
macro_rules! Depcrate_automatonimpl_49 {
() => {
// Module: crate::automaton
// Provides: {"impl_49"}
// Dependencies: {}
impl < 'a , 'h , A : Automaton > Iterator for FindIter < 'a , 'h , A > { type Item = Match ; # [inline (always)] fn next (& mut self) -> Option < Match > { let mut m = self . search () ? ; if m . is_empty () { m = self . handle_overlapping_empty_match (m) ? ; } self . input . set_start (m . end ()) ; self . last_match_end = Some (m . end ()) ; Some (m) } }
};
}
