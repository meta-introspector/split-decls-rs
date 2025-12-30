// Generated macro for StateIter (struct)
macro_rules! Depcrate_dfa_denseStateIter {
() => {
// Module: crate::dfa::dense
// Provides: {"StateIter"}
// Dependencies: {}
# [doc = " An iterator over all states in a DFA."] # [doc = ""] # [doc = " This iterator yields a tuple for each state. The first element of the"] # [doc = " tuple corresponds to a state's identifier, and the second element"] # [doc = " corresponds to the state itself (comprised of its transitions)."] # [doc = ""] # [doc = " `'a` corresponding to the lifetime of original DFA, `T` corresponds to"] # [doc = " the type of the transition table itself."] pub (crate) struct StateIter < 'a , T > { tt : & 'a TransitionTable < T > , it : iter :: Enumerate < slice :: Chunks < 'a , StateID > > , }
};
}
