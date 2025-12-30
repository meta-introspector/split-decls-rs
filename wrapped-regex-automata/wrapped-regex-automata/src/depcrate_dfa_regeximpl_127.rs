// Generated macro for impl_127 (impl)
macro_rules! Depcrate_dfa_regeximpl_127 {
() => {
// Module: crate::dfa::regex
// Provides: {"impl_127"}
// Dependencies: {}
impl < 'r , 'h , A : Automaton > Iterator for FindMatches < 'r , 'h , A > { type Item = Match ; # [inline] fn next (& mut self) -> Option < Match > { let FindMatches { re , ref mut it } = * self ; it . advance (| input | re . try_search (input)) } }
};
}
