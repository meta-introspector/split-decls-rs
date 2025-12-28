macro_rules! deps {
    () => {
        DFA!();
        MatchError!();
        Cache!();
        Input!();
        LazyStateID!();
    };
}

macro_rules! init_rev {
    () => {
        deps!();
        # [cfg_attr (feature = "perf-inline" , inline (always))] fn init_rev (dfa : & DFA , cache : & mut Cache , input : & Input < '_ > ,) -> Result < LazyStateID , MatchError > { let sid = dfa . start_state_reverse (cache , input) ? ; debug_assert ! (! sid . is_match ()) ; Ok (sid) }
    };
}

init_rev!();