// Generated macro for MoveDepGraph (struct)
macro_rules! Depcrate_errorsMoveDepGraph {
() => {
// Module: crate::errors
// Provides: {"MoveDepGraph"}
// Dependencies: {}
# [derive (Diagnostic)] # [diag (incremental_move_dep_graph)] pub (crate) struct MoveDepGraph < 'a > { pub from : & 'a Path , pub to : & 'a Path , pub err : std :: io :: Error , }
};
}
