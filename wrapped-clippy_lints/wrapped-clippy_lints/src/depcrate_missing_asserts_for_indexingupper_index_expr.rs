// Generated macro for upper_index_expr (function)
macro_rules! Depcrate_missing_asserts_for_indexingupper_index_expr {
() => {
// Module: crate::missing_asserts_for_indexing
// Provides: {"upper_index_expr"}
// Dependencies: {}
# [doc = " Extracts the upper index of a slice indexing expression."] # [doc = ""] # [doc = " E.g. for `5` this returns `Some(5)`, for `..5` this returns `Some(4)`,"] # [doc = " for `..=5` this returns `Some(5)`"] fn upper_index_expr (cx : & LateContext < '_ > , expr : & Expr < '_ >) -> Option < usize > { if let ExprKind :: Lit (lit) = & expr . kind && let LitKind :: Int (Pu128 (index) , _) = lit . node { Some (index as usize) } else if let Some (Range { end : Some (end) , limits , .. }) = Range :: hir (cx , expr) && let ExprKind :: Lit (lit) = & end . kind && let LitKind :: Int (Pu128 (index @ 1 ..) , _) = lit . node { match limits { RangeLimits :: HalfOpen => Some (index as usize - 1) , RangeLimits :: Closed => Some (index as usize) , } } else { None } }
};
}
