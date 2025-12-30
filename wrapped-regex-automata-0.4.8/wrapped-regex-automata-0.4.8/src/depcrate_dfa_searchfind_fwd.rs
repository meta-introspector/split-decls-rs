// Generated macro for find_fwd (function)
macro_rules! Depcrate_dfa_searchfind_fwd {
() => {
// Module: crate::dfa::search
// Provides: {"find_fwd"}
// Dependencies: {}
# [inline (never)] pub fn find_fwd < A : Automaton + ? Sized > (dfa : & A , input : & Input < '_ > ,) -> Result < Option < HalfMatch > , MatchError > { if input . is_done () { return Ok (None) ; } let pre = if input . get_anchored () . is_anchored () { None } else { dfa . get_prefilter () } ; if pre . is_some () { if input . get_earliest () { find_fwd_imp (dfa , input , pre , true) } else { find_fwd_imp (dfa , input , pre , false) } } else { if input . get_earliest () { find_fwd_imp (dfa , input , None , true) } else { find_fwd_imp (dfa , input , None , false) } } }
};
}
