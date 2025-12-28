macro_rules! deps {
    () => {
        DFAEngine!();
    };
}

macro_rules! DFA {
    () => {
        deps!();
        # [derive (Debug)] pub (crate) struct DFA (Option < DFAEngine >) ;
    };
}

DFA!()