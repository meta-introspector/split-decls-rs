macro_rules! deps {
    () => {
        Prefilter!();
        ReverseDFA!();
        ReverseHybrid!();
        Core!();
        NFA!();
    };
}

macro_rules! ReverseInner {
    () => {
        deps!();
        # [derive (Debug)] struct ReverseInner { core : Core , preinner : Prefilter , nfarev : NFA , hybrid : wrappers :: ReverseHybrid , dfa : wrappers :: ReverseDFA , }
    };
}

ReverseInner!();