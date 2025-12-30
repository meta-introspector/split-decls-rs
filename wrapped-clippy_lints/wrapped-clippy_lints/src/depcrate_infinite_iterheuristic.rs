// Generated macro for Heuristic (enum)
macro_rules! Depcrate_infinite_iterHeuristic {
() => {
// Module: crate::infinite_iter
// Provides: {"Heuristic"}
// Dependencies: {}
# [doc = " This tells us what to look for to know if the iterator returned by"] # [doc = " this method is infinite"] # [derive (Copy , Clone)] enum Heuristic { # [doc = " infinite no matter what"] Always , # [doc = " infinite if the first argument is"] First , # [doc = " infinite if any of the supplied arguments is"] Any , # [doc = " infinite if all of the supplied arguments are"] All , }
};
}
