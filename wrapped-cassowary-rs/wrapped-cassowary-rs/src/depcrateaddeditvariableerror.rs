// Generated macro for AddEditVariableError (enum)
macro_rules! DepcrateAddEditVariableError {
() => {
// Module: crate
// Provides: {"AddEditVariableError"}
// Dependencies: {}
# [doc = " The possible error conditions that `Solver::add_edit_variable` can fail with."] # [derive (Debug , Copy , Clone)] pub enum AddEditVariableError { # [doc = " The specified variable is already marked as an edit variable in the solver."] DuplicateEditVariable , # [doc = " The specified strength was `REQUIRED`. This is illegal for edit variable strengths."] BadRequiredStrength }
};
}
