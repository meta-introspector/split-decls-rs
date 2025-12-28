macro_rules! deps {
    () => {
        DFA!();
        NFA!();
    };
}

macro_rules! SENTINEL_STATES {
    () => {
        deps!();
        # [doc = " The number of \"sentinel\" states that get added to every lazy DFA."] # [doc = ""] # [doc = " These are special states indicating status conditions of a search: unknown,"] # [doc = " dead and quit. These states in particular also use zero NFA states, so"] # [doc = " their memory usage is quite small. This is relevant for computing the"] # [doc = " minimum memory needed for a lazy DFA cache."] const SENTINEL_STATES : usize = 3 ;
    };
}

SENTINEL_STATES!();