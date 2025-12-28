macro_rules! deps {
    () => {
        Input!();
        Cache!();
        DFA!();
        RetryError!();
        MatchError!();
        RetryQuadraticError!();
        HalfMatch!();
    };
}

macro_rules! hybrid_try_search_half_rev {
    () => {
        deps!();
        # [cfg (feature = "hybrid")] pub (crate) fn hybrid_try_search_half_rev (dfa : & crate :: hybrid :: dfa :: DFA , cache : & mut crate :: hybrid :: dfa :: Cache , input : & Input < '_ > , min_start : usize ,) -> Result < Option < HalfMatch > , RetryError > { let mut mat = None ; let mut sid = dfa . start_state_reverse (cache , input) ? ; if input . start () == input . end () { hybrid_eoi_rev (dfa , cache , input , & mut sid , & mut mat) ? ; return Ok (mat) ; } let mut at = input . end () - 1 ; loop { sid = dfa . next_state (cache , sid , input . haystack () [at]) . map_err (| _ | MatchError :: gave_up (at)) ? ; if sid . is_tagged () { if sid . is_match () { let pattern = dfa . match_pattern (cache , sid , 0) ; mat = Some (HalfMatch :: new (pattern , at + 1)) ; } else if sid . is_dead () { return Ok (mat) ; } else if sid . is_quit () { return Err (MatchError :: quit (input . haystack () [at] , at) . into ()) ; } } if at == input . start () { break ; } at -= 1 ; if at < min_start { trace ! ("reached position {at} which is before the previous literal \
				 match, quitting to avoid quadratic behavior" ,) ; return Err (RetryError :: Quadratic (RetryQuadraticError :: new ())) ; } } let was_dead = sid . is_dead () ; hybrid_eoi_rev (dfa , cache , input , & mut sid , & mut mat) ? ; if at == input . start () && mat . map_or (false , | m | m . offset () > input . start ()) && ! was_dead { trace ! ("reached beginning of search at offset {at} without hitting \
             a dead state, quitting to avoid potential false positive match" ,) ; return Err (RetryError :: Quadratic (RetryQuadraticError :: new ())) ; } Ok (mat) }
    };
}

hybrid_try_search_half_rev!()