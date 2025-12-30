// Generated macro for AddConstraintError (enum)
macro_rules! DepcrateAddConstraintError {
() => {
// Module: crate
// Provides: {"AddConstraintError"}
// Dependencies: {}
# [doc = " The possible error conditions that `Solver::add_constraint` can fail with."] # [derive (Debug , Copy , Clone)] pub enum AddConstraintError { # [doc = " The constraint specified has already been added to the solver."] DuplicateConstraint , # [doc = " The constraint is required, but it is unsatisfiable in conjunction with the existing constraints."] UnsatisfiableConstraint , # [doc = " The solver entered an invalid state. If this occurs please report the issue. This variant specifies"] # [doc = " additional details as a string."] InternalSolverError (& 'static str) }
};
}
