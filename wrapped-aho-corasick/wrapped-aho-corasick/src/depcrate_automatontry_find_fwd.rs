// Generated macro for try_find_fwd (function)
macro_rules! Depcrate_automatontry_find_fwd {
() => {
// Module: crate::automaton
// Provides: {"try_find_fwd"}
// Dependencies: {}
# [inline (never)] pub (crate) fn try_find_fwd < A : Automaton + ? Sized > (aut : & A , input : & Input < '_ > ,) -> Result < Option < Match > , MatchError > { if input . is_done () { return Ok (None) ; } let earliest = aut . match_kind () . is_standard () || input . get_earliest () ; if input . get_anchored () . is_anchored () { try_find_fwd_imp (aut , input , None , Anchored :: Yes , earliest) } else if let Some (pre) = aut . prefilter () { if earliest { try_find_fwd_imp (aut , input , Some (pre) , Anchored :: No , true) } else { try_find_fwd_imp (aut , input , Some (pre) , Anchored :: No , false) } } else { if earliest { try_find_fwd_imp (aut , input , None , Anchored :: No , true) } else { try_find_fwd_imp (aut , input , None , Anchored :: No , false) } } }
};
}
