// Generated macro for hash_stmt (function)
macro_rules! Depcrate_hir_utilshash_stmt {
() => {
// Module: crate::hir_utils
// Provides: {"hash_stmt"}
// Dependencies: {}
pub fn hash_stmt (cx : & LateContext < '_ > , s : & Stmt < '_ >) -> u64 { let mut h = SpanlessHash :: new (cx) ; h . hash_stmt (s) ; h . finish () }
};
}
