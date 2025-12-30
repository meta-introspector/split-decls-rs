// Generated macro for parse_call (function)
macro_rules! Depcrate_manual_string_newparse_call {
() => {
// Module: crate::manual_string_new
// Provides: {"parse_call"}
// Dependencies: {}
# [doc = " Tries to parse an expression as a function call, emitting the warning if necessary."] fn parse_call (cx : & LateContext < '_ > , span : Span , func : & Expr < '_ > , arg : & Expr < '_ >) { if let ExprKind :: Path (qpath) = & func . kind { if let QPath :: TypeRelative (ty , path_seg) = qpath && [sym :: from , sym :: try_from] . contains (& path_seg . ident . name) && let TyKind :: Path (qpath) = & ty . kind && let QPath :: Resolved (_ , path) = qpath && let [path_seg] = path . segments && path_seg . ident . name == sym :: String && is_expr_kind_empty_str (& arg . kind) { warn_then_suggest (cx , span) ; } else if let QPath :: Resolved (_ , path) = qpath && let [path_seg1 , path_seg2] = path . segments && is_expr_kind_empty_str (& arg . kind) && ((path_seg1 . ident . name == sym :: From && path_seg2 . ident . name == sym :: from) || (path_seg1 . ident . name == sym :: TryFrom && path_seg2 . ident . name == sym :: try_from)) { warn_then_suggest (cx , span) ; } } }
};
}
