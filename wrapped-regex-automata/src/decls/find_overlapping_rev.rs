macro_rules! deps {
    () => {
        OverlappingState!();
        DFA!();
        MatchError!();
        Cache!();
        Input!();
        HalfMatch!();
    };
}

macro_rules! find_overlapping_rev {
    () => {
        deps!();
        # [inline (never)] pub (crate) fn find_overlapping_rev (dfa : & DFA , cache : & mut Cache , input : & Input < '_ > , state : & mut OverlappingState ,) -> Result < () , MatchError > { state . mat = None ; if input . is_done () { return Ok (()) ; } let mut sid = match state . id { None => { let sid = init_rev (dfa , cache , input) ? ; state . id = Some (sid) ; if input . start () == input . end () { state . rev_eoi = true ; } else { state . at = input . end () - 1 ; } sid } Some (sid) => { if let Some (match_index) = state . next_match_index { let match_len = dfa . match_len (cache , sid) ; if match_index < match_len { state . next_match_index = Some (match_index + 1) ; let pattern = dfa . match_pattern (cache , sid , match_index) ; state . mat = Some (HalfMatch :: new (pattern , state . at)) ; return Ok (()) ; } } if state . rev_eoi { return Ok (()) ; } else if state . at == input . start () { state . rev_eoi = true ; } else { state . at -= 1 ; } sid } } ; cache . search_start (state . at) ; while ! state . rev_eoi { sid = dfa . next_state (cache , sid , input . haystack () [state . at]) . map_err (| _ | gave_up (state . at)) ? ; if sid . is_tagged () { state . id = Some (sid) ; if sid . is_start () { } else if sid . is_match () { state . next_match_index = Some (1) ; let pattern = dfa . match_pattern (cache , sid , 0) ; state . mat = Some (HalfMatch :: new (pattern , state . at + 1)) ; cache . search_finish (state . at) ; return Ok (()) ; } else if sid . is_dead () { cache . search_finish (state . at) ; return Ok (()) ; } else if sid . is_quit () { cache . search_finish (state . at) ; return Err (MatchError :: quit (input . haystack () [state . at] , state . at ,)) ; } else { debug_assert ! (sid . is_unknown ()) ; unreachable ! ("sid being unknown is a bug") ; } } if state . at == input . start () { break ; } state . at -= 1 ; cache . search_update (state . at) ; } let result = eoi_rev (dfa , cache , input , & mut sid , & mut state . mat) ; state . rev_eoi = true ; state . id = Some (sid) ; if state . mat . is_some () { state . next_match_index = Some (1) ; } cache . search_finish (input . start ()) ; result }
    };
}

find_overlapping_rev!();