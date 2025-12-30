// Generated macro for PathState (enum)
macro_rules! Depcrate_pathPathState {
() => {
// Module: crate::path
// Provides: {"PathState"}
// Dependencies: {}
# [doc = " The different states of the path validation."] # [derive (Debug , Copy , Clone , PartialEq , Eq , PartialOrd , Ord)] pub enum PathState { # [doc = " The path failed its validation."] Failed , # [doc = " The path exists, but no path validation has been performed."] Unknown , # [doc = " The path is under validation."] Validating , # [doc = " The remote address has been validated, but not the path MTU."] ValidatingMTU , # [doc = " The path has been validated."] Validated , }
};
}
