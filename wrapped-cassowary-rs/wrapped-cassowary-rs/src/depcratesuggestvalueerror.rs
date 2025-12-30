// Generated macro for SuggestValueError (enum)
macro_rules! DepcrateSuggestValueError {
() => {
// Module: crate
// Provides: {"SuggestValueError"}
// Dependencies: {}
# [doc = " The possible error conditions that `Solver::suggest_value` can fail with."] # [derive (Debug , Copy , Clone)] pub enum SuggestValueError { # [doc = " The specified variable was not an edit variable in the solver, so cannot have its value suggested."] UnknownEditVariable , # [doc = " The solver entered an invalid state. If this occurs please report the issue. This variant specifies"] # [doc = " additional details as a string."] InternalSolverError (& 'static str) }
};
}
