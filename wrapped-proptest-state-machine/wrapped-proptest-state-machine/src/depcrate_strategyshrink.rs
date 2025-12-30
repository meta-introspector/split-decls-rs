// Generated macro for Shrink (enum)
macro_rules! Depcrate_strategyShrink {
() => {
// Module: crate::strategy
// Provides: {"Shrink"}
// Dependencies: {}
# [doc = " A shrinking operation"] # [derive (Clone , Copy , Debug)] enum Shrink { # [doc = " Shrink the initial state"] InitialState , # [doc = " Delete a transition at given index"] DeleteTransition (usize) , # [doc = " Shrink a transition at given index"] Transition (usize) , }
};
}
