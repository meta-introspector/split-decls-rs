// Generated macro for find_fwd (function)
macro_rules! Depcrate_hybrid_searchfind_fwd {
() => {
// Module: crate::hybrid::search
// Provides: {"find_fwd"}
// Dependencies: {}
# [inline (never)] pub (crate) fn find_fwd (dfa : & DFA , cache : & mut Cache , input : & Input < '_ > ,) -> Result < Option < HalfMatch > , MatchError > { if input . is_done () { return Ok (None) ; } let pre = if input . get_anchored () . is_anchored () { None } else { dfa . get_config () . get_prefilter () } ; if pre . is_some () { if input . get_earliest () { find_fwd_imp (dfa , cache , input , pre , true) } else { find_fwd_imp (dfa , cache , input , pre , false) } } else { if input . get_earliest () { find_fwd_imp (dfa , cache , input , None , true) } else { find_fwd_imp (dfa , cache , input , None , false) } } }
};
}
