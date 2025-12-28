macro_rules! deps {
    () => {
        DFA!();
        MatchError!();
        HalfMatch!();
        StateID!();
        Input!();
        Automaton!();
    };
}

macro_rules! dfa_eoi_fwd {
    () => {
        deps!();
        # [cfg (feature = "dfa-build")] # [cfg_attr (feature = "perf-inline" , inline (always))] fn dfa_eoi_fwd (dfa : & crate :: dfa :: dense :: DFA < alloc :: vec :: Vec < u32 > > , input : & Input < '_ > , sid : & mut crate :: util :: primitives :: StateID , mat : & mut Option < HalfMatch > ,) -> Result < () , MatchError > { use crate :: dfa :: Automaton ; let sp = input . get_span () ; match input . haystack () . get (sp . end) { Some (& b) => { * sid = dfa . next_state (* sid , b) ; if dfa . is_match_state (* sid) { let pattern = dfa . match_pattern (* sid , 0) ; * mat = Some (HalfMatch :: new (pattern , sp . end)) ; } else if dfa . is_quit_state (* sid) { return Err (MatchError :: quit (b , sp . end)) ; } } None => { * sid = dfa . next_eoi_state (* sid) ; if dfa . is_match_state (* sid) { let pattern = dfa . match_pattern (* sid , 0) ; * mat = Some (HalfMatch :: new (pattern , input . haystack () . len ())) ; } debug_assert ! (! dfa . is_quit_state (* sid)) ; } } Ok (()) }
    };
}

dfa_eoi_fwd!();