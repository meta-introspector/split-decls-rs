macro_rules! deps {
    () => {
        Input!();
        MatchError!();
        DFA!();
        Cache!();
        LazyStateID!();
    };
}

macro_rules! init_fwd {
    () => {
        deps!();
        # [cfg_attr (feature = "perf-inline" , inline (always))] fn init_fwd (dfa : & DFA , cache : & mut Cache , input : & Input < '_ > ,) -> Result < LazyStateID , MatchError > { let sid = dfa . start_state_forward (cache , input) ? ; debug_assert ! (! sid . is_match ()) ; Ok (sid) }
    };
}

init_fwd!()