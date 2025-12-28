macro_rules! deps {
    () => {
        DFA!();
    };
}

macro_rules! MIN_STATES {
    () => {
        deps!();
        # [doc = " The minimum number of states that a lazy DFA's cache size must support."] # [doc = ""] # [doc = " This is checked at time of construction to ensure that at least some small"] # [doc = " number of states can fit in the given capacity allotment. If we can't fit"] # [doc = " at least this number of states, then the thinking is that it's pretty"] # [doc = " senseless to use the lazy DFA. More to the point, parts of the code do"] # [doc = " assume that the cache can fit at least some small number of states."] const MIN_STATES : usize = SENTINEL_STATES + 2 ;
    };
}

MIN_STATES!();