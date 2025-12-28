macro_rules! automaton {
    () => {
        # [cfg (feature = "dfa-search")] mod automaton ;
    };
}

automaton!()