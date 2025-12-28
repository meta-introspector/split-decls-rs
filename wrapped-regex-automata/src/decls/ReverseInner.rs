macro_rules! deps {
    () => {
        ReverseHybrid!();
        ReverseDFA!();
        Core!();
        NFA!();
        Prefilter!();
    };
}

macro_rules! ReverseInner {
    () => {
        deps!();
        # [derive (Debug)] struct ReverseInner { core : Core , preinner : Prefilter , nfarev : NFA , hybrid : wrappers :: ReverseHybrid , dfa : wrappers :: ReverseDFA , }
    };
}

ReverseInner!()