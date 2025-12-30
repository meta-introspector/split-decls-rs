// Generated macro for MoveSite (struct)
macro_rules! Depcrate_diagnostics_conflict_errorsMoveSite {
() => {
// Module: crate::diagnostics::conflict_errors
// Provides: {"MoveSite"}
// Dependencies: {}
# [derive (Debug)] struct MoveSite { # [doc = " Index of the \"move out\" that we found. The `MoveData` can"] # [doc = " then tell us where the move occurred."] moi : MoveOutIndex , # [doc = " `true` if we traversed a back edge while walking from the point"] # [doc = " of error to the move site."] traversed_back_edge : bool , }
};
}
