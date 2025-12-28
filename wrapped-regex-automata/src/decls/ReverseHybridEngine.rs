macro_rules! deps {
    () => {
        DFA!();
    };
}

macro_rules! ReverseHybridEngine {
    () => {
        deps!();
        # [derive (Debug)] pub (crate) struct ReverseHybridEngine (# [cfg (feature = "hybrid")] hybrid :: dfa :: DFA , # [cfg (not (feature = "hybrid"))] () ,) ;
    };
}

ReverseHybridEngine!()