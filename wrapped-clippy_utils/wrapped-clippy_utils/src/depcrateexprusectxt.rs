// Generated macro for ExprUseCtxt (struct)
macro_rules! DepcrateExprUseCtxt {
() => {
// Module: crate
// Provides: {"ExprUseCtxt"}
// Dependencies: {}
# [doc = " The context an expressions value is used in."] pub struct ExprUseCtxt < 'tcx > { # [doc = " The parent node which consumes the value."] pub node : Node < 'tcx > , # [doc = " The child id of the node the value came from."] pub child_id : HirId , # [doc = " Any adjustments applied to the type."] pub adjustments : & 'tcx [Adjustment < 'tcx >] , # [doc = " Whether the type must unify with another code path."] pub is_ty_unified : bool , # [doc = " Whether the value will be moved before it's used."] pub moved_before_use : bool , # [doc = " Whether the use site has the same `SyntaxContext` as the value."] pub same_ctxt : bool , }
};
}
