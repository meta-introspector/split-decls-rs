// Generated macro for find_overlapping_fwd (function)
macro_rules! Depcrate_hybrid_searchfind_overlapping_fwd {
() => {
// Module: crate::hybrid::search
// Provides: {"find_overlapping_fwd"}
// Dependencies: {}
# [inline (never)] pub (crate) fn find_overlapping_fwd (dfa : & DFA , cache : & mut Cache , input : & Input < '_ > , state : & mut OverlappingState ,) -> Result < () , MatchError > { state . mat = None ; if input . is_done () { return Ok (()) ; } let pre = if input . get_anchored () . is_anchored () { None } else { dfa . get_config () . get_prefilter () } ; if pre . is_some () { find_overlapping_fwd_imp (dfa , cache , input , pre , state) } else { find_overlapping_fwd_imp (dfa , cache , input , None , state) } }
};
}
