// Generated macro for dfa_try_search_half_rev (function)
macro_rules! Depcrate_meta_limiteddfa_try_search_half_rev {
() => {
// Module: crate::meta::limited
// Provides: {"dfa_try_search_half_rev"}
// Dependencies: {}
# [cfg (feature = "dfa-build")] pub (crate) fn dfa_try_search_half_rev (dfa : & crate :: dfa :: dense :: DFA < alloc :: vec :: Vec < u32 > > , input : & Input < '_ > , min_start : usize ,) -> Result < Option < HalfMatch > , RetryError > { use crate :: dfa :: Automaton ; let mut mat = None ; let mut sid = dfa . start_state_reverse (input) ? ; if input . start () == input . end () { dfa_eoi_rev (dfa , input , & mut sid , & mut mat) ? ; return Ok (mat) ; } let mut at = input . end () - 1 ; loop { sid = dfa . next_state (sid , input . haystack () [at]) ; if dfa . is_special_state (sid) { if dfa . is_match_state (sid) { let pattern = dfa . match_pattern (sid , 0) ; mat = Some (HalfMatch :: new (pattern , at + 1)) ; } else if dfa . is_dead_state (sid) { return Ok (mat) ; } else if dfa . is_quit_state (sid) { return Err (MatchError :: quit (input . haystack () [at] , at) . into ()) ; } } if at == input . start () { break ; } at -= 1 ; if at < min_start { trace ! ("reached position {at} which is before the previous literal \
				 match, quitting to avoid quadratic behavior" ,) ; return Err (RetryError :: Quadratic (RetryQuadraticError :: new ())) ; } } let was_dead = dfa . is_dead_state (sid) ; dfa_eoi_rev (dfa , input , & mut sid , & mut mat) ? ; if at == input . start () && mat . map_or (false , | m | m . offset () > input . start ()) && ! was_dead { trace ! ("reached beginning of search at offset {at} without hitting \
             a dead state, quitting to avoid potential false positive match" ,) ; return Err (RetryError :: Quadratic (RetryQuadraticError :: new ())) ; } Ok (mat) }
};
}
