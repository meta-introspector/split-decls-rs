// Generated macro for TransitionState (enum)
macro_rules! Depcrate_strategyTransitionState {
() => {
// Module: crate::strategy
// Provides: {"TransitionState"}
// Dependencies: {}
# [doc = " The state of a transition in the model"] # [derive (Clone , Copy , Debug)] enum TransitionState { # [doc = " The transition that is equal to the result of `ValueTree::current()`"] # [doc = " and satisfies the pre-conditions"] Accepted , # [doc = " The transition has been simplified, but rejected by pre-conditions"] SimplifyRejected , # [doc = " The transition has been complicated, but rejected by pre-conditions"] ComplicateRejected , }
};
}
