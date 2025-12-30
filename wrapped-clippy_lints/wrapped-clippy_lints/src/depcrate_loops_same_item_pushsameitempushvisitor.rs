// Generated macro for SameItemPushVisitor (struct)
macro_rules! Depcrate_loops_same_item_pushSameItemPushVisitor {
() => {
// Module: crate::loops::same_item_push
// Provides: {"SameItemPushVisitor"}
// Dependencies: {}
struct SameItemPushVisitor < 'a , 'tcx > { non_deterministic_expr : bool , multiple_pushes : bool , vec_push : Option < (& 'tcx Expr < 'tcx > , & 'tcx Expr < 'tcx > , SyntaxContext) > , cx : & 'a LateContext < 'tcx > , used_locals : FxHashSet < HirId > , }
};
}
