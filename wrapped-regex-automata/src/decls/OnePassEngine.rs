macro_rules! deps {
    () => {
        DFA!();
    };
}

macro_rules! OnePassEngine {
    () => {
        deps!();
        # [derive (Debug)] pub (crate) struct OnePassEngine (# [cfg (feature = "dfa-onepass")] onepass :: DFA , # [cfg (not (feature = "dfa-onepass"))] () ,) ;
    };
}

OnePassEngine!();