// Generated macro for RemoveConstraintError (enum)
macro_rules! DepcrateRemoveConstraintError {
() => {
// Module: crate
// Provides: {"RemoveConstraintError"}
// Dependencies: {}
# [doc = " The possible error conditions that `Solver::remove_constraint` can fail with."] # [derive (Debug , Copy , Clone)] pub enum RemoveConstraintError { # [doc = " The constraint specified was not already in the solver, so cannot be removed."] UnknownConstraint , # [doc = " The solver entered an invalid state. If this occurs please report the issue. This variant specifies"] # [doc = " additional details as a string."] InternalSolverError (& 'static str) }
};
}
