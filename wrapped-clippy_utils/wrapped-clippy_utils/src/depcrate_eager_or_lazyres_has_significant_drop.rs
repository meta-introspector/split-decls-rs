// Generated macro for res_has_significant_drop (function)
macro_rules! Depcrate_eager_or_lazyres_has_significant_drop {
() => {
// Module: crate::eager_or_lazy
// Provides: {"res_has_significant_drop"}
// Dependencies: {}
fn res_has_significant_drop (res : Res , cx : & LateContext < '_ > , e : & Expr < '_ >) -> bool { if let Res :: Def (DefKind :: Ctor (..) | DefKind :: Variant | DefKind :: Enum | DefKind :: Struct , _) | Res :: SelfCtor (_) | Res :: SelfTyAlias { .. } = res { cx . typeck_results () . expr_ty (e) . has_significant_drop (cx . tcx , cx . typing_env ()) } else { false } }
};
}
