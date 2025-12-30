// Generated macro for DfaState (struct)
macro_rules! Depcrate_readerDfaState {
() => {
// Module: crate::reader
// Provides: {"DfaState"}
// Dependencies: {}
# [doc = " A single DFA state."] # [doc = ""] # [doc = " A DFA state is represented by the starting index of its corresponding row"] # [doc = " in the DFA transition table. This representation allows us to elide a"] # [doc = " single multiplication instruction when computing the next transition for"] # [doc = " a particular input byte."] # [derive (Clone , Copy , Debug , Eq , Ord , PartialEq , PartialOrd)] struct DfaState (u8) ;
};
}
