// Generated macro for impl_518 (impl)
macro_rules! Depcrate_nfa_thompson_backtrackimpl_518 {
() => {
// Module: crate::nfa::thompson::backtrack
// Provides: {"impl_518"}
// Dependencies: {}
impl < 'r , 'c , 'h > Iterator for TryCapturesMatches < 'r , 'c , 'h > { type Item = Result < Captures , MatchError > ; # [inline] fn next (& mut self) -> Option < Result < Captures , MatchError > > { let TryCapturesMatches { re , ref mut cache , ref mut caps , ref mut it } = * self ; let _ = it . try_advance (| input | { re . try_search (cache , input , caps) ? ; Ok (caps . get_match ()) }) . transpose () ? ; if caps . is_match () { Some (Ok (caps . clone ())) } else { None } } }
};
}
