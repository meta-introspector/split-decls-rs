// Generated macro for impl_11538 (impl)
macro_rules! Depcrate_zero_repeat_side_effectsimpl_11538 {
() => {
// Module: crate::zero_repeat_side_effects
// Provides: {"impl_11538"}
// Dependencies: {}
impl LateLintPass < '_ > for ZeroRepeatSideEffects { fn check_expr (& mut self , cx : & LateContext < '_ > , expr : & rustc_hir :: Expr < '_ >) { if let Some (args) = VecArgs :: hir (cx , expr) && let VecArgs :: Repeat (inner_expr , len) = args && let ExprKind :: Lit (l) = len . kind && let LitKind :: Int (Pu128 (0) , _) = l . node { inner_check (cx , expr , inner_expr , true) ; } else if let ExprKind :: Repeat (inner_expr , const_arg) = expr . kind && let ConstArgKind :: Anon (anon_const) = const_arg . kind && let length_expr = cx . tcx . hir_body (anon_const . body) . value && ! length_expr . span . from_expansion () && let ExprKind :: Lit (literal) = length_expr . kind && let LitKind :: Int (Pu128 (0) , _) = literal . node { inner_check (cx , expr , inner_expr , false) ; } } }
};
}
