// Generated macro for StateTransitionIter (struct)
macro_rules! Depcrate_dfa_denseStateTransitionIter {
() => {
// Module: crate::dfa::dense
// Provides: {"StateTransitionIter"}
// Dependencies: {}
# [doc = " An iterator over all transitions in a single DFA state. This yields"] # [doc = " a number of transitions equivalent to the alphabet length of the"] # [doc = " corresponding DFA."] # [doc = ""] # [doc = " Each transition is represented by a tuple. The first element is the input"] # [doc = " byte for that transition and the second element is the transition itself."] # [derive (Debug)] pub (crate) struct StateTransitionIter < 'a > { len : usize , it : iter :: Enumerate < slice :: Iter < 'a , StateID > > , }
};
}
