macro_rules! deps {
    () => {
        Transition!();
    };
}

macro_rules! SparseTransitions {
    () => {
        deps!();
        # [doc = " A sequence of transitions used to represent a sparse state."] # [doc = ""] # [doc = " This is the primary representation of a [`Sparse`](State::Sparse) state."] # [doc = " It corresponds to a sorted sequence of transitions with non-overlapping"] # [doc = " byte ranges. If the byte at the current position in the haystack matches"] # [doc = " one of the byte ranges, then the finite state machine should take the"] # [doc = " corresponding transition."] # [derive (Clone , Debug , Eq , PartialEq)] pub struct SparseTransitions { # [doc = " The sorted sequence of non-overlapping transitions."] pub transitions : Box < [Transition] > , }
    };
}

SparseTransitions!();