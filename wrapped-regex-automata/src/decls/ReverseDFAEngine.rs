macro_rules! deps {
    () => {
        DFA!();
    };
}

macro_rules! ReverseDFAEngine {
    () => {
        deps!();
        # [derive (Debug)] pub (crate) struct ReverseDFAEngine (# [cfg (feature = "dfa-build")] dfa :: dense :: DFA < Vec < u32 > > , # [cfg (not (feature = "dfa-build"))] () ,) ;
    };
}

ReverseDFAEngine!()