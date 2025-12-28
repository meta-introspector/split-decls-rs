macro_rules! deps {
    () => {
        NFA!();
        StateID!();
    };
}

macro_rules! ThompsonRef {
    () => {
        deps!();
        # [doc = " A value that represents the result of compiling a sub-expression of a"] # [doc = " regex's HIR. Specifically, this represents a sub-graph of the NFA that"] # [doc = " has an initial state at `start` and a final state at `end`."] # [derive (Clone , Copy , Debug)] pub (crate) struct ThompsonRef { pub (crate) start : StateID , pub (crate) end : StateID , }
    };
}

ThompsonRef!()