macro_rules! deps {
    () => {
        ReverseDFAEngine!();
    };
}

macro_rules! ReverseDFA {
    () => {
        deps!();
        # [derive (Debug)] pub (crate) struct ReverseDFA (Option < ReverseDFAEngine >) ;
    };
}

ReverseDFA!();