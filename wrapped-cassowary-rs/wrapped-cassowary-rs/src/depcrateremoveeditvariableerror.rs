// Generated macro for RemoveEditVariableError (enum)
macro_rules! DepcrateRemoveEditVariableError {
() => {
// Module: crate
// Provides: {"RemoveEditVariableError"}
// Dependencies: {}
# [doc = " The possible error conditions that `Solver::remove_edit_variable` can fail with."] # [derive (Debug , Copy , Clone)] pub enum RemoveEditVariableError { # [doc = " The specified variable was not an edit variable in the solver, so cannot be removed."] UnknownEditVariable , # [doc = " The solver entered an invalid state. If this occurs please report the issue. This variant specifies"] # [doc = " additional details as a string."] InternalSolverError (& 'static str) }
};
}
