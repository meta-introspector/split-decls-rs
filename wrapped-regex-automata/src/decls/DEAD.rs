macro_rules! deps {
    () => {
        StateID!();
        DFA!();
    };
}

macro_rules! DEAD {
    () => {
        deps!();
        # [doc = " This is an alias for a state ID of zero. It has special significance"] # [doc = " because it always corresponds to the first state in a DFA, and the first"] # [doc = " state in a DFA is always \"dead.\" That is, the dead state always has all"] # [doc = " of its transitions set to itself. Moreover, the dead state is used as a"] # [doc = " sentinel for various things. e.g., In search, reaching a dead state means"] # [doc = " that the search must stop."] const DEAD : crate :: util :: primitives :: StateID = crate :: util :: primitives :: StateID :: ZERO ;
    };
}

DEAD!()