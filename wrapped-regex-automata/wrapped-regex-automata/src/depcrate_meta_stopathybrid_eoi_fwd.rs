// Generated macro for hybrid_eoi_fwd (function)
macro_rules! Depcrate_meta_stopathybrid_eoi_fwd {
() => {
// Module: crate::meta::stopat
// Provides: {"hybrid_eoi_fwd"}
// Dependencies: {}
# [cfg (feature = "hybrid")] # [cfg_attr (feature = "perf-inline" , inline (always))] fn hybrid_eoi_fwd (dfa : & crate :: hybrid :: dfa :: DFA , cache : & mut crate :: hybrid :: dfa :: Cache , input : & Input < '_ > , sid : & mut crate :: hybrid :: LazyStateID , mat : & mut Option < HalfMatch > ,) -> Result < () , MatchError > { let sp = input . get_span () ; match input . haystack () . get (sp . end) { Some (& b) => { * sid = dfa . next_state (cache , * sid , b) . map_err (| _ | MatchError :: gave_up (sp . end)) ? ; if sid . is_match () { let pattern = dfa . match_pattern (cache , * sid , 0) ; * mat = Some (HalfMatch :: new (pattern , sp . end)) ; } else if sid . is_quit () { return Err (MatchError :: quit (b , sp . end)) ; } } None => { * sid = dfa . next_eoi_state (cache , * sid) . map_err (| _ | MatchError :: gave_up (input . haystack () . len ())) ? ; if sid . is_match () { let pattern = dfa . match_pattern (cache , * sid , 0) ; * mat = Some (HalfMatch :: new (pattern , input . haystack () . len ())) ; } debug_assert ! (! sid . is_quit ()) ; } } Ok (()) }
};
}
