// Generated macro for WhichCaptures (enum)
macro_rules! Depcrate_nfa_thompson_compilerWhichCaptures {
() => {
// Module: crate::nfa::thompson::compiler
// Provides: {"WhichCaptures"}
// Dependencies: {}
# [doc = " A configuration indicating which kinds of"] # [doc = " [`State::Capture`](crate::nfa::thompson::State::Capture) states to include."] # [doc = ""] # [doc = " This configuration can be used with [`Config::which_captures`] to control"] # [doc = " which capture states are compiled into a Thompson NFA."] # [doc = ""] # [doc = " The default configuration is [`WhichCaptures::All`]."] # [derive (Clone , Copy , Debug)] pub enum WhichCaptures { # [doc = " All capture states, including those corresponding to both implicit and"] # [doc = " explicit capture groups, are included in the Thompson NFA."] All , # [doc = " Only capture states corresponding to implicit capture groups are"] # [doc = " included. Implicit capture groups appear in every pattern implicitly"] # [doc = " and correspond to the overall match of a pattern."] # [doc = ""] # [doc = " This is useful when one only cares about the overall match of a"] # [doc = " pattern. By excluding capture states from explicit capture groups,"] # [doc = " one might be able to reduce the memory usage of a multi-pattern regex"] # [doc = " substantially if it was otherwise written to have many explicit capture"] # [doc = " groups."] Implicit , # [doc = " No capture states are compiled into the Thompson NFA."] # [doc = ""] # [doc = " This is useful when capture states are either not needed (for example,"] # [doc = " if one is only trying to build a DFA) or if they aren't supported (for"] # [doc = " example, a reverse NFA)."] None , }
};
}
