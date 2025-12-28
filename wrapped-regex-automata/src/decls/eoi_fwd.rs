macro_rules! deps {
    () => {
        Cache!();
        LazyStateID!();
        Input!();
        MatchError!();
        DFA!();
        HalfMatch!();
    };
}

macro_rules! eoi_fwd {
    () => {
        deps!();
        # [cfg_attr (feature = "perf-inline" , inline (always))] fn eoi_fwd (dfa : & DFA , cache : & mut Cache , input : & Input < '_ > , sid : & mut LazyStateID , mat : & mut Option < HalfMatch > ,) -> Result < () , MatchError > { let sp = input . get_span () ; match input . haystack () . get (sp . end) { Some (& b) => { * sid = dfa . next_state (cache , * sid , b) . map_err (| _ | gave_up (sp . end)) ? ; if sid . is_match () { let pattern = dfa . match_pattern (cache , * sid , 0) ; * mat = Some (HalfMatch :: new (pattern , sp . end)) ; } else if sid . is_quit () { return Err (MatchError :: quit (b , sp . end)) ; } } None => { * sid = dfa . next_eoi_state (cache , * sid) . map_err (| _ | gave_up (input . haystack () . len ())) ? ; if sid . is_match () { let pattern = dfa . match_pattern (cache , * sid , 0) ; * mat = Some (HalfMatch :: new (pattern , input . haystack () . len ())) ; } debug_assert ! (! sid . is_quit ()) ; } } Ok (()) }
    };
}

eoi_fwd!();