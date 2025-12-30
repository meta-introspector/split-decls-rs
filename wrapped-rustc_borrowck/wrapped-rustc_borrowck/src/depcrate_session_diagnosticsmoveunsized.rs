// Generated macro for MoveUnsized (struct)
macro_rules! Depcrate_session_diagnosticsMoveUnsized {
() => {
// Module: crate::session_diagnostics
// Provides: {"MoveUnsized"}
// Dependencies: {}
# [derive (Diagnostic)] # [diag (borrowck_move_unsized , code = E0161)] pub (crate) struct MoveUnsized < 'tcx > { pub ty : Ty < 'tcx > , # [primary_span] # [label] pub span : Span , }
};
}
