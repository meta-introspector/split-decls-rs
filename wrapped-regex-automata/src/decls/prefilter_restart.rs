macro_rules! deps {
    () => {
        MatchError!();
        Input!();
        DFA!();
        LazyStateID!();
        Cache!();
    };
}

macro_rules! prefilter_restart {
    () => {
        deps!();
        # [doc = " Re-compute the starting state that a DFA should be in after finding a"] # [doc = " prefilter candidate match at the position `at`."] # [doc = ""] # [doc = " It is always correct to call this, but not always necessary. Namely,"] # [doc = " whenever the DFA has a universal start state, the DFA can remain in the"] # [doc = " start state that it was in when it ran the prefilter. Why? Because in that"] # [doc = " case, there is only one start state."] # [doc = ""] # [doc = " When does a DFA have a universal start state? In precisely cases where"] # [doc = " it has no look-around assertions in its prefix. So for example, `\\bfoo`"] # [doc = " does not have a universal start state because the start state depends on"] # [doc = " whether the byte immediately before the start position is a word byte or"] # [doc = " not. However, `foo\\b` does have a universal start state because the word"] # [doc = " boundary does not appear in the pattern's prefix."] # [doc = ""] # [doc = " So... most cases don't need this, but when a pattern doesn't have a"] # [doc = " universal start state, then after a prefilter candidate has been found, the"] # [doc = " current state *must* be re-litigated as if computing the start state at the"] # [doc = " beginning of the search because it might change. That is, not all start"] # [doc = " states are created equal."] # [doc = ""] # [doc = " Why avoid it? Because while it's not super expensive, it isn't a trivial"] # [doc = " operation to compute the start state. It is much better to avoid it and"] # [doc = " just state in the current state if you know it to be correct."] # [cfg_attr (feature = "perf-inline" , inline (always))] fn prefilter_restart (dfa : & DFA , cache : & mut Cache , input : & Input < '_ > , at : usize ,) -> Result < LazyStateID , MatchError > { let mut input = input . clone () ; input . set_start (at) ; init_fwd (dfa , cache , & input) }
    };
}

prefilter_restart!();