// Generated macro for impl_635 (impl)
macro_rules! Depcrate_nfa_thompson_pikevmimpl_635 {
() => {
// Module: crate::nfa::thompson::pikevm
// Provides: {"impl_635"}
// Dependencies: {}
impl < 'r , 'c , 'h > Iterator for CapturesMatches < 'r , 'c , 'h > { type Item = Captures ; # [inline] fn next (& mut self) -> Option < Captures > { let CapturesMatches { re , ref mut cache , ref mut caps , ref mut it } = * self ; it . advance (| input | { re . search (cache , input , caps) ; Ok (caps . get_match ()) }) ; if caps . is_match () { Some (caps . clone ()) } else { None } } }
};
}
