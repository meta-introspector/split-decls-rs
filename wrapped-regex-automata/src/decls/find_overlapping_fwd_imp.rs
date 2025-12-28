macro_rules! deps {
    () => {
        DFA!();
        Prefilter!();
        Cache!();
        Input!();
        MatchError!();
        OverlappingState!();
        Span!();
        HalfMatch!();
    };
}

macro_rules! find_overlapping_fwd_imp {
    () => {
        deps!();
        # [cfg_attr (feature = "perf-inline" , inline (always))] fn find_overlapping_fwd_imp (dfa : & DFA , cache : & mut Cache , input : & Input < '_ > , pre : Option < & '_ Prefilter > , state : & mut OverlappingState ,) -> Result < () , MatchError > { let universal_start = dfa . get_nfa () . look_set_prefix_any () . is_empty () ; let mut sid = match state . id { None => { state . at = input . start () ; init_fwd (dfa , cache , input) ? } Some (sid) => { if let Some (match_index) = state . next_match_index { let match_len = dfa . match_len (cache , sid) ; if match_index < match_len { state . next_match_index = Some (match_index + 1) ; let pattern = dfa . match_pattern (cache , sid , match_index) ; state . mat = Some (HalfMatch :: new (pattern , state . at)) ; return Ok (()) ; } } state . at += 1 ; if state . at > input . end () { return Ok (()) ; } sid } } ; cache . search_start (state . at) ; while state . at < input . end () { sid = dfa . next_state (cache , sid , input . haystack () [state . at]) . map_err (| _ | gave_up (state . at)) ? ; if sid . is_tagged () { state . id = Some (sid) ; if sid . is_start () { if let Some (ref pre) = pre { let span = Span :: from (state . at .. input . end ()) ; match pre . find (input . haystack () , span) { None => return Ok (()) , Some (ref span) => { if span . start > state . at { state . at = span . start ; if ! universal_start { sid = prefilter_restart (dfa , cache , & input , state . at ,) ? ; } continue ; } } } } } else if sid . is_match () { state . next_match_index = Some (1) ; let pattern = dfa . match_pattern (cache , sid , 0) ; state . mat = Some (HalfMatch :: new (pattern , state . at)) ; cache . search_finish (state . at) ; return Ok (()) ; } else if sid . is_dead () { cache . search_finish (state . at) ; return Ok (()) ; } else if sid . is_quit () { cache . search_finish (state . at) ; return Err (MatchError :: quit (input . haystack () [state . at] , state . at ,)) ; } else { debug_assert ! (sid . is_unknown ()) ; unreachable ! ("sid being unknown is a bug") ; } } state . at += 1 ; cache . search_update (state . at) ; } let result = eoi_fwd (dfa , cache , input , & mut sid , & mut state . mat) ; state . id = Some (sid) ; if state . mat . is_some () { state . next_match_index = Some (1) ; } cache . search_finish (input . end ()) ; result }
    };
}

find_overlapping_fwd_imp!();