// Generated macro for impl_10981 (impl)
macro_rules! Depcrate_vecimpl_10981 {
() => {
// Module: crate::vec
// Provides: {"impl_10981"}
// Dependencies: {}
impl UselessVec { # [doc = " Checks if the surrounding environment requires this expression to actually be of type"] # [doc = " `Vec<_>`, or if it can be changed to `&[]`/`[]` without causing type errors."] fn expr_usage_requires_vec (& mut self , cx : & LateContext < '_ > , expr : & Expr < '_ >) -> VecToArray { match cx . tcx . parent_hir_node (expr . hir_id) { Node :: LetStmt (LetStmt { ty : None , pat : Pat { kind : PatKind :: Binding (_ , id , ..) , .. } , .. }) => { let only_slice_uses = for_each_local_use_after_expr (cx , * id , expr . hir_id , | expr | { if let Some (parent) = get_parent_expr (cx , expr) && (adjusts_to_slice (cx , expr) || matches ! (parent . kind , ExprKind :: Index (..)) || is_allowed_vec_method (cx , parent)) { ControlFlow :: Continue (()) } else { ControlFlow :: Break (()) } }) . is_continue () ; if only_slice_uses { VecToArray :: Possible } else { VecToArray :: Impossible } } , Node :: LetStmt (LetStmt { ty : Some (_) , .. }) if higher :: VecArgs :: hir (cx , expr) . is_some () => { VecToArray :: Impossible } , Node :: Expr (Expr { span , .. }) if span . is_desugaring (DesugaringKind :: ForLoop) && self . msrv . meets (cx , msrvs :: ARRAY_INTO_ITERATOR) => { VecToArray :: Possible } , _ => { if adjusts_to_slice (cx , expr) { VecToArray :: Possible } else { VecToArray :: Impossible } } , } } }
};
}
