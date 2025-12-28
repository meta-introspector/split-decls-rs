macro_rules! deps {
    () => {
        StateID!();
        DFA!();
        NFA!();
        SmallIndex!();
        NonMaxUsize!();
        PikeVM!();
    };
}

macro_rules! FollowEpsilon {
    () => {
        deps!();
        # [doc = " Represents a stack frame for use while computing an epsilon closure."] # [doc = ""] # [doc = " (An \"epsilon closure\" refers to the set of reachable NFA states from a"] # [doc = " single state without consuming any input. That is, the set of all epsilon"] # [doc = " transitions not only from that single state, but from every other state"] # [doc = " reachable by an epsilon transition as well. This is why it's called a"] # [doc = " \"closure.\" Computing an epsilon closure is also done during DFA"] # [doc = " determinization! Compare and contrast the epsilon closure here in this"] # [doc = " PikeVM and the one used for determinization in crate::util::determinize.)"] # [doc = ""] # [doc = " Computing the epsilon closure in a Thompson NFA proceeds via a depth"] # [doc = " first traversal over all epsilon transitions from a particular state."] # [doc = " (A depth first traversal is important because it emulates the same priority"] # [doc = " of matches that is typically found in backtracking regex engines.) This"] # [doc = " depth first traversal is naturally expressed using recursion, but to avoid"] # [doc = " a call stack size proportional to the size of a regex, we put our stack on"] # [doc = " the heap instead."] # [doc = ""] # [doc = " This stack thus consists of call frames. The typical call frame is"] # [doc = " `Explore`, which instructs epsilon closure to explore the epsilon"] # [doc = " transitions from that state. (Subsequent epsilon transitions are then"] # [doc = " pushed on to the stack as more `Explore` frames.) If the state ID being"] # [doc = " explored has no epsilon transitions, then the capturing group slots are"] # [doc = " copied from the original state that sparked the epsilon closure (from the"] # [doc = " 'step' routine) to the state ID being explored. This way, capturing group"] # [doc = " slots are forwarded from the previous state to the next."] # [doc = ""] # [doc = " The other stack frame, `RestoreCaptures`, instructs the epsilon closure to"] # [doc = " set the position for a particular slot back to some particular offset. This"] # [doc = " frame is pushed when `Explore` sees a `Capture` transition. `Explore` will"] # [doc = " set the offset of the slot indicated in `Capture` to the current offset,"] # [doc = " and then push the old offset on to the stack as a `RestoreCapture` frame."] # [doc = " Thus, the new offset is only used until the epsilon closure reverts back to"] # [doc = " the `RestoreCapture` frame. In effect, this gives the `Capture` epsilon"] # [doc = " transition its \"scope\" to only states that come \"after\" it during depth"] # [doc = " first traversal."] # [derive (Clone , Debug)] enum FollowEpsilon { # [doc = " Explore the epsilon transitions from a state ID."] Explore (StateID) , # [doc = " Reset the given `slot` to the given `offset` (which might be `None`)."] RestoreCapture { slot : SmallIndex , offset : Option < NonMaxUsize > } , }
    };
}

FollowEpsilon!();