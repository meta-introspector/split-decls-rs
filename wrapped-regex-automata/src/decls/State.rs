macro_rules! deps {
    () => {
        DFA!();
        NFA!();
    };
}

macro_rules! State {
    () => {
        deps!();
        # [doc = " A DFA state that, at its core, is represented by an ordered set of NFA"] # [doc = " states."] # [doc = ""] # [doc = " This type is intended to be used only in NFA-to-DFA conversion via powerset"] # [doc = " construction."] # [doc = ""] # [doc = " It may be cheaply cloned and accessed safely from multiple threads"] # [doc = " simultaneously."] # [derive (Clone , Eq , Hash , PartialEq , PartialOrd , Ord)] pub (crate) struct State (Arc < [u8] >) ;
    };
}

State!();