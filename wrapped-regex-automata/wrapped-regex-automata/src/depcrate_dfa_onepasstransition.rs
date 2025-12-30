// Generated macro for Transition (struct)
macro_rules! Depcrate_dfa_onepassTransition {
() => {
// Module: crate::dfa::onepass
// Provides: {"Transition"}
// Dependencies: {}
# [doc = " Represents a single transition in a one-pass DFA."] # [doc = ""] # [doc = " The high 21 bits corresponds to the state ID. The bit following corresponds"] # [doc = " to the special \"match wins\" flag. The remaining low 42 bits corresponds to"] # [doc = " the transition epsilons, which contains the slots that should be saved when"] # [doc = " this transition is followed and the conditional epsilon transitions that"] # [doc = " must be satisfied in order to follow this transition."] # [derive (Clone , Copy , Eq , PartialEq)] struct Transition (u64) ;
};
}
