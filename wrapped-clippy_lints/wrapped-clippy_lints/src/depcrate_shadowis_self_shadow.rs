// Generated macro for is_self_shadow (function)
macro_rules! Depcrate_shadowis_self_shadow {
() => {
// Module: crate::shadow
// Provides: {"is_self_shadow"}
// Dependencies: {}
# [doc = " Returns true if the expression is a simple transformation of a local binding such as `&x`"] fn is_self_shadow (cx : & LateContext < '_ > , pat : & Pat < '_ > , mut expr : & Expr < '_ > , hir_id : HirId) -> bool { let is_direct_binding = cx . tcx . hir_parent_iter (pat . hir_id) . map_while (| (_id , node) | match node { Node :: Pat (pat) => Some (pat) , _ => None , }) . all (| pat | matches ! (pat . kind , PatKind :: Ref (..) | PatKind :: Or (_))) ; if ! is_direct_binding { return false ; } loop { expr = match expr . kind { ExprKind :: AddrOf (_ , _ , e) | ExprKind :: Block (& Block { stmts : [] , expr : Some (e) , .. } , _ ,) | ExprKind :: Unary (UnOp :: Deref , e) => e , ExprKind :: Path (QPath :: Resolved (None , path)) => break path . res == Res :: Local (hir_id) , _ => break false , } } }
};
}
