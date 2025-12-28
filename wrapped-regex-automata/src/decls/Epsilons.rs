macro_rules! deps {
    () => {
        NFA!();
        Transition!();
        DFA!();
        StateID!();
    };
}

macro_rules! Epsilons {
    () => {
        deps!();
        # [doc = " Epsilons represents all of the NFA epsilons transitions that went into a"] # [doc = " single transition in a single DFA state. In this case, it only represents"] # [doc = " the epsilon transitions that have some kind of non-consuming side effect:"] # [doc = " either the transition requires storing the current position of the search"] # [doc = " into a slot, or the transition is conditional and requires the current"] # [doc = " position in the input to satisfy an assertion before the transition may be"] # [doc = " taken."] # [doc = ""] # [doc = " This folds the cumulative effect of a group of NFA states (all connected"] # [doc = " by epsilon transitions) down into a single set of bits. While these bits"] # [doc = " can represent all possible conditional epsilon transitions, it only permits"] # [doc = " storing up to a somewhat small number of slots."] # [doc = ""] # [doc = " Epsilons is represented as a 42-bit integer. For example, it is packed into"] # [doc = " the lower 42 bits of a `Transition`. (Where the high 22 bits contains a"] # [doc = " `StateID` and a special \"match wins\" property.)"] # [derive (Clone , Copy)] struct Epsilons (u64) ;
    };
}

Epsilons!()