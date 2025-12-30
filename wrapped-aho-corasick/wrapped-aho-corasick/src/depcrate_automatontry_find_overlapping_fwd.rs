// Generated macro for try_find_overlapping_fwd (function)
macro_rules! Depcrate_automatontry_find_overlapping_fwd {
() => {
// Module: crate::automaton
// Provides: {"try_find_overlapping_fwd"}
// Dependencies: {}
# [inline (never)] fn try_find_overlapping_fwd < A : Automaton + ? Sized > (aut : & A , input : & Input < '_ > , state : & mut OverlappingState ,) -> Result < () , MatchError > { state . mat = None ; if input . is_done () { return Ok (()) ; } if aut . prefilter () . is_some () && ! input . get_anchored () . is_anchored () { let pre = aut . prefilter () . unwrap () ; try_find_overlapping_fwd_imp (aut , input , Some (pre) , state) } else { try_find_overlapping_fwd_imp (aut , input , None , state) } }
};
}
