// Generated macro for ThompsonRef (struct)
macro_rules! Depcrate_nfaThompsonRef {
() => {
// Module: crate::nfa
// Provides: {"ThompsonRef"}
// Dependencies: {}
# [doc = " A value that represents the result of compiling a sub-expression of a"] # [doc = " regex's HIR. Specifically, this represents a sub-graph of the NFA that"] # [doc = " has an initial state at `start` and a final state at `end`."] # [derive (Clone , Copy , Debug)] struct ThompsonRef { start : StateID , end : StateID , }
};
}
