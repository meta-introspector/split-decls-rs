// Generated macro for impl_66 (impl)
macro_rules! Depcrate_dfa_denseimpl_66 {
() => {
// Module: crate::dfa::dense
// Provides: {"impl_66"}
// Dependencies: {}
impl < 'a > Iterator for StateSparseTransitionIter < 'a > { type Item = (alphabet :: Unit , alphabet :: Unit , StateID) ; fn next (& mut self) -> Option < (alphabet :: Unit , alphabet :: Unit , StateID) > { while let Some ((unit , next)) = self . dense . next () { let (prev_start , prev_end , prev_next) = match self . cur { Some (t) => t , None => { self . cur = Some ((unit , unit , next)) ; continue ; } } ; if prev_next == next && ! unit . is_eoi () { self . cur = Some ((prev_start , unit , prev_next)) ; } else { self . cur = Some ((unit , unit , next)) ; if prev_next != DEAD { return Some ((prev_start , prev_end , prev_next)) ; } } } if let Some ((start , end , next)) = self . cur . take () { if next != DEAD { return Some ((start , end , next)) ; } } None } }
};
}
