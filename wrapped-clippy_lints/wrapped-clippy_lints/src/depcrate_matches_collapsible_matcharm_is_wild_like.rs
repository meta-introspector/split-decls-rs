// Generated macro for arm_is_wild_like (function)
macro_rules! Depcrate_matches_collapsible_matcharm_is_wild_like {
() => {
// Module: crate::matches::collapsible_match
// Provides: {"arm_is_wild_like"}
// Dependencies: {}
# [doc = " A \"wild-like\" arm has a wild (`_`) or `None` pattern and no guard. Such arms can be \"collapsed\""] # [doc = " into a single wild arm without any significant loss in semantics or readability."] fn arm_is_wild_like (cx : & LateContext < '_ > , arm : & Arm < '_ >) -> bool { if arm . guard . is_some () { return false ; } match arm . pat . kind { PatKind :: Binding (..) | PatKind :: Wild => true , PatKind :: Expr (PatExpr { kind : PatExprKind :: Path (qpath) , hir_id , .. }) => cx . qpath_res (qpath , * hir_id) . ctor_parent (cx) . is_lang_item (cx , OptionNone) , _ => false , } }
};
}
