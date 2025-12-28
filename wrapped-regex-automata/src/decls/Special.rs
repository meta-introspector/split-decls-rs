macro_rules! deps {
    () => {
        StateID!();
        DFA!();
    };
}

macro_rules! Special {
    () => {
        deps!();
        # [derive (Clone , Copy , Debug)] pub (crate) struct Special { # [doc = " The identifier of the last special state in a DFA. A state is special"] # [doc = " if and only if its identifier is less than or equal to `max`."] pub (crate) max : StateID , # [doc = " The identifier of the quit state in a DFA. (There is no analogous field"] # [doc = " for the dead state since the dead state's ID is always zero, regardless"] # [doc = " of state ID size.)"] pub (crate) quit_id : StateID , # [doc = " The identifier of the first match state."] pub (crate) min_match : StateID , # [doc = " The identifier of the last match state."] pub (crate) max_match : StateID , # [doc = " The identifier of the first accelerated state."] pub (crate) min_accel : StateID , # [doc = " The identifier of the last accelerated state."] pub (crate) max_accel : StateID , # [doc = " The identifier of the first start state."] pub (crate) min_start : StateID , # [doc = " The identifier of the last start state."] pub (crate) max_start : StateID , }
    };
}

Special!()