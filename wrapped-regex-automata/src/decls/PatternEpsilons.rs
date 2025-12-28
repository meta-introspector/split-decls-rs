macro_rules! deps {
    () => {
        NFA!();
        DFA!();
    };
}

macro_rules! PatternEpsilons {
    () => {
        deps!();
        # [doc = " A representation of a match state's pattern ID along with the epsilons for"] # [doc = " when a match occurs."] # [doc = ""] # [doc = " A match state in a one-pass DFA, unlike in a more general DFA, has exactly"] # [doc = " one pattern ID. If it had more, then the original NFA would not have been"] # [doc = " one-pass."] # [doc = ""] # [doc = " The \"epsilons\" part of this corresponds to what was found in the epsilon"] # [doc = " transitions between the transition taken in the last byte of input and the"] # [doc = " ultimate match state. This might include saving slots and/or conditional"] # [doc = " epsilon transitions that must be satisfied before one can report the match."] # [doc = ""] # [doc = " Technically, every state has room for a 'PatternEpsilons', but it is only"] # [doc = " ever non-empty for match states."] # [derive (Clone , Copy)] struct PatternEpsilons (u64) ;
    };
}

PatternEpsilons!();