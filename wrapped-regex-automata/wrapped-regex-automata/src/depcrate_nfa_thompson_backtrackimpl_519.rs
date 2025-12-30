// Generated macro for impl_519 (impl)
macro_rules! Depcrate_nfa_thompson_backtrackimpl_519 {
() => {
// Module: crate::nfa::thompson::backtrack
// Provides: {"impl_519"}
// Dependencies: {}
impl < 'r , 'c , 'h > Iterator for TryFindMatches < 'r , 'c , 'h > { type Item = Result < Match , MatchError > ; # [inline] fn next (& mut self) -> Option < Result < Match , MatchError > > { let TryFindMatches { re , ref mut cache , ref mut caps , ref mut it } = * self ; it . try_advance (| input | { re . try_search (cache , input , caps) ? ; Ok (caps . get_match ()) }) . transpose () } }
};
}
