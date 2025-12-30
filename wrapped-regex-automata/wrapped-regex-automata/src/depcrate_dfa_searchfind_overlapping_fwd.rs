// Generated macro for find_overlapping_fwd (function)
macro_rules! Depcrate_dfa_searchfind_overlapping_fwd {
() => {
// Module: crate::dfa::search
// Provides: {"find_overlapping_fwd"}
// Dependencies: {}
# [inline (never)] pub fn find_overlapping_fwd < A : Automaton + ? Sized > (dfa : & A , input : & Input < '_ > , state : & mut OverlappingState ,) -> Result < () , MatchError > { state . mat = None ; if input . is_done () { return Ok (()) ; } let pre = if input . get_anchored () . is_anchored () { None } else { dfa . get_prefilter () } ; if pre . is_some () { find_overlapping_fwd_imp (dfa , input , pre , state) } else { find_overlapping_fwd_imp (dfa , input , None , state) } }
};
}
