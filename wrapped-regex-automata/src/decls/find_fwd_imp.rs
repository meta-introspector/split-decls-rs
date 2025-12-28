macro_rules! deps {
    () => {
        Cache!();
        HalfMatch!();
        Input!();
        MatchError!();
        Span!();
        DFA!();
        Prefilter!();
    };
}

macro_rules! find_fwd_imp {
    () => {
        deps!();
        # [cfg_attr (feature = "perf-inline" , inline (always))] fn find_fwd_imp (dfa : & DFA , cache : & mut Cache , input : & Input < '_ > , pre : Option < & '_ Prefilter > , earliest : bool ,) -> Result < Option < HalfMatch > , MatchError > { let universal_start = dfa . get_nfa () . look_set_prefix_any () . is_empty () ; let mut mat = None ; let mut sid = init_fwd (dfa , cache , input) ? ; let mut at = input . start () ; macro_rules ! next_unchecked { ($ sid : expr , $ at : expr) => { { let byte = * input . haystack () . get_unchecked ($ at) ; dfa . next_state_untagged_unchecked (cache , $ sid , byte) } } ; } if let Some (ref pre) = pre { let span = Span :: from (at .. input . end ()) ; match pre . find (input . haystack () , span) { None => return Ok (mat) , Some (ref span) => { at = span . start ; if ! universal_start { sid = prefilter_restart (dfa , cache , & input , at) ? ; } } } } cache . search_start (at) ; while at < input . end () { if sid . is_tagged () { cache . search_update (at) ; sid = dfa . next_state (cache , sid , input . haystack () [at]) . map_err (| _ | gave_up (at)) ? ; } else { let mut prev_sid = sid ; while at < input . end () { prev_sid = unsafe { next_unchecked ! (sid , at) } ; if prev_sid . is_tagged () || at + 3 >= input . end () { core :: mem :: swap (& mut prev_sid , & mut sid) ; break ; } at += 1 ; sid = unsafe { next_unchecked ! (prev_sid , at) } ; if sid . is_tagged () { break ; } at += 1 ; prev_sid = unsafe { next_unchecked ! (sid , at) } ; if prev_sid . is_tagged () { core :: mem :: swap (& mut prev_sid , & mut sid) ; break ; } at += 1 ; sid = unsafe { next_unchecked ! (prev_sid , at) } ; if sid . is_tagged () { break ; } at += 1 ; } if sid . is_unknown () { cache . search_update (at) ; sid = dfa . next_state (cache , prev_sid , input . haystack () [at]) . map_err (| _ | gave_up (at)) ? ; } } if sid . is_tagged () { if sid . is_start () { if let Some (ref pre) = pre { let span = Span :: from (at .. input . end ()) ; match pre . find (input . haystack () , span) { None => { cache . search_finish (span . end) ; return Ok (mat) ; } Some (ref span) => { if span . start > at { at = span . start ; if ! universal_start { sid = prefilter_restart (dfa , cache , & input , at ,) ? ; } continue ; } } } } } else if sid . is_match () { let pattern = dfa . match_pattern (cache , sid , 0) ; mat = Some (HalfMatch :: new (pattern , at)) ; if earliest { cache . search_finish (at) ; return Ok (mat) ; } } else if sid . is_dead () { cache . search_finish (at) ; return Ok (mat) ; } else if sid . is_quit () { cache . search_finish (at) ; return Err (MatchError :: quit (input . haystack () [at] , at)) ; } else { debug_assert ! (sid . is_unknown ()) ; unreachable ! ("sid being unknown is a bug") ; } } at += 1 ; } eoi_fwd (dfa , cache , input , & mut sid , & mut mat) ? ; cache . search_finish (input . end ()) ; Ok (mat) }
    };
}

find_fwd_imp!();