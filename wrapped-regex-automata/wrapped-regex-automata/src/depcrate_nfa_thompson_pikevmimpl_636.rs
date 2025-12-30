// Generated macro for impl_636 (impl)
macro_rules! Depcrate_nfa_thompson_pikevmimpl_636 {
() => {
// Module: crate::nfa::thompson::pikevm
// Provides: {"impl_636"}
// Dependencies: {}
impl < 'r , 'c , 'h > Iterator for FindMatches < 'r , 'c , 'h > { type Item = Match ; # [inline] fn next (& mut self) -> Option < Match > { let FindMatches { re , ref mut cache , ref mut caps , ref mut it } = * self ; it . advance (| input | { re . search (cache , input , caps) ; Ok (caps . get_match ()) }) } }
};
}
