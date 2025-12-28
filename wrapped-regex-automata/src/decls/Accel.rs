macro_rules! deps {
    () => {
        DFA!();
    };
}

macro_rules! Accel {
    () => {
        deps!();
        # [doc = " Accel represents a structure for determining how to \"accelerate\" a DFA"] # [doc = " state."] # [doc = ""] # [doc = " Namely, it contains zero or more bytes that must be seen in order for the"] # [doc = " DFA to leave the state it is associated with. In practice, the actual range"] # [doc = " is 1 to 3 bytes."] # [doc = ""] # [doc = " The purpose of acceleration is to identify states whose vast majority"] # [doc = " of transitions are just loops back to the same state. For example,"] # [doc = " in the regex `(?-u)^[^a]+b`, the corresponding DFA will have a state"] # [doc = " (corresponding to `[^a]+`) where all transitions *except* for `a` and"] # [doc = " `b` loop back to itself. Thus, this state can be \"accelerated\" by simply"] # [doc = " looking for the next occurrence of either `a` or `b` instead of explicitly"] # [doc = " following transitions. (In this case, `b` transitions to the next state"] # [doc = " where as `a` would transition to the dead state.)"] # [derive (Clone)] pub (crate) struct Accel { # [doc = " The first byte is the length. Subsequent bytes are the accelerated"] # [doc = " bytes."] # [doc = ""] # [doc = " Note that we make every accelerator 8 bytes as a slightly wasteful"] # [doc = " way of making sure alignment is always correct for state ID sizes of"] # [doc = " 1, 2, 4 and 8. This should be okay since accelerated states aren't"] # [doc = " particularly common, especially when Unicode is enabled."] bytes : [u8 ; ACCEL_CAP] , }
    };
}

Accel!()