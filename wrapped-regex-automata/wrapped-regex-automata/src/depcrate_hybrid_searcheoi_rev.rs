// Generated macro for eoi_rev (function)
macro_rules! Depcrate_hybrid_searcheoi_rev {
() => {
// Module: crate::hybrid::search
// Provides: {"eoi_rev"}
// Dependencies: {}
# [cfg_attr (feature = "perf-inline" , inline (always))] fn eoi_rev (dfa : & DFA , cache : & mut Cache , input : & Input < '_ > , sid : & mut LazyStateID , mat : & mut Option < HalfMatch > ,) -> Result < () , MatchError > { let sp = input . get_span () ; if sp . start > 0 { let byte = input . haystack () [sp . start - 1] ; * sid = dfa . next_state (cache , * sid , byte) . map_err (| _ | gave_up (sp . start)) ? ; if sid . is_match () { let pattern = dfa . match_pattern (cache , * sid , 0) ; * mat = Some (HalfMatch :: new (pattern , sp . start)) ; } else if sid . is_quit () { return Err (MatchError :: quit (byte , sp . start - 1)) ; } } else { * sid = dfa . next_eoi_state (cache , * sid) . map_err (| _ | gave_up (sp . start)) ? ; if sid . is_match () { let pattern = dfa . match_pattern (cache , * sid , 0) ; * mat = Some (HalfMatch :: new (pattern , 0)) ; } debug_assert ! (! sid . is_quit ()) ; } Ok (()) }
};
}
