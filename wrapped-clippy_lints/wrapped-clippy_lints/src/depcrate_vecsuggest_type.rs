// Generated macro for suggest_type (function)
macro_rules! Depcrate_vecsuggest_type {
() => {
// Module: crate::vec
// Provides: {"suggest_type"}
// Dependencies: {}
fn suggest_type (expr : & Expr < '_ >) -> SuggestedType { if let ExprKind :: AddrOf (BorrowKind :: Ref , mutability , _) = expr . kind { SuggestedType :: SliceRef (mutability) } else { SuggestedType :: Array } }
};
}
