// Generated macro for find_bool_lit (function)
macro_rules! Depcrate_matches_match_like_matchesfind_bool_lit {
() => {
// Module: crate::matches::match_like_matches
// Provides: {"find_bool_lit"}
// Dependencies: {}
# [doc = " Extract a `bool` or `{ bool }`"] fn find_bool_lit (ex : & Expr < '_ >) -> Option < bool > { match ex . kind { ExprKind :: Lit (Spanned { node : LitKind :: Bool (b) , .. }) => Some (b) , ExprKind :: Block (rustc_hir :: Block { stmts : [] , expr : Some (exp) , .. } , _ ,) => { if let ExprKind :: Lit (Spanned { node : LitKind :: Bool (b) , .. }) = exp . kind { Some (b) } else { None } } , _ => None , } }
};
}
