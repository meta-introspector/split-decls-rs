macro_rules! deps {
    () => {
        BoundedBacktracker!();
        OnePass!();
        Hybrid!();
        DFA!();
        RegexInfo!();
        NFA!();
        PikeVM!();
        Prefilter!();
    };
}

macro_rules! Core {
    () => {
        deps!();
        # [derive (Debug)] struct Core { info : RegexInfo , pre : Option < Prefilter > , nfa : NFA , nfarev : Option < NFA > , pikevm : wrappers :: PikeVM , backtrack : wrappers :: BoundedBacktracker , onepass : wrappers :: OnePass , hybrid : wrappers :: Hybrid , dfa : wrappers :: DFA , }
    };
}

Core!()