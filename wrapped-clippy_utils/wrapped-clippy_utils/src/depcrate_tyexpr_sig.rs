// Generated macro for expr_sig (function)
macro_rules! Depcrate_tyexpr_sig {
() => {
// Module: crate::ty
// Provides: {"expr_sig"}
// Dependencies: {}
# [doc = " If the expression is function like, get the signature for it."] pub fn expr_sig < 'tcx > (cx : & LateContext < 'tcx > , expr : & Expr < '_ >) -> Option < ExprFnSig < 'tcx > > { if let Res :: Def (DefKind :: Fn | DefKind :: Ctor (_ , CtorKind :: Fn) | DefKind :: AssocFn , id) = expr . res (cx) { Some (ExprFnSig :: Sig (cx . tcx . fn_sig (id) . instantiate_identity () , Some (id))) } else { ty_sig (cx , cx . typeck_results () . expr_ty_adjusted (expr) . peel_refs ()) } }
};
}
