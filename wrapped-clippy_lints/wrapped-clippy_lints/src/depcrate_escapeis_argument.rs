// Generated macro for is_argument (function)
macro_rules! Depcrate_escapeis_argument {
() => {
// Module: crate::escape
// Provides: {"is_argument"}
// Dependencies: {}
fn is_argument (tcx : TyCtxt < '_ > , id : HirId) -> bool { match tcx . hir_node (id) { Node :: Pat (Pat { kind : PatKind :: Binding (..) , .. }) => () , _ => return false , } matches ! (tcx . parent_hir_node (id) , Node :: Param (_)) }
};
}
