macro_rules! deps {
    () => {
        State!();
        StateID!();
        NFA!();
        SmallIndex!();
    };
}

macro_rules! impl_85 {
    () => {
        deps!();
        impl State { # [doc = " Return true if and only if this state is a match state."] pub (crate) fn is_match (& self) -> bool { self . matches != StateID :: ZERO } # [doc = " Returns the failure transition for this state."] pub (crate) fn fail (& self) -> StateID { self . fail } # [doc = " Returns the depth of this state. That is, the number of transitions"] # [doc = " this state is from the start state of the NFA."] pub (crate) fn depth (& self) -> SmallIndex { self . depth } }
    };
}

impl_85!();