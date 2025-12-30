// Generated macro for LazyAutomaton (trait)
macro_rules! Depcrate_lazy_automatonLazyAutomaton {
() => {
// Module: crate::lazy_automaton
// Provides: {"LazyAutomaton"}
// Dependencies: {}
pub trait LazyAutomaton : Automaton { fn matches_earliest_fwd_lazy < S : Writeable + ? Sized > (& self , haystack : & S) -> bool ; }
};
}
