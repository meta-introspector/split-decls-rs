// Generated macro for hash_expr (function)
macro_rules! Depcrate_hir_utilshash_expr {
() => {
// Module: crate::hir_utils
// Provides: {"hash_expr"}
// Dependencies: {}
pub fn hash_expr (cx : & LateContext < '_ > , e : & Expr < '_ >) -> u64 { let mut h = SpanlessHash :: new (cx) ; h . hash_expr (e) ; h . finish () }
};
}
