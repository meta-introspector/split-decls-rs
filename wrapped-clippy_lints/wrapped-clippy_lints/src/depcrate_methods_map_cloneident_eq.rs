// Generated macro for ident_eq (function)
macro_rules! Depcrate_methods_map_cloneident_eq {
() => {
// Module: crate::methods::map_clone
// Provides: {"ident_eq"}
// Dependencies: {}
fn ident_eq (name : Ident , path : & hir :: Expr < '_ >) -> bool { if let hir :: ExprKind :: Path (hir :: QPath :: Resolved (None , path)) = path . kind { path . segments . len () == 1 && path . segments [0] . ident == name } else { false } }
};
}
