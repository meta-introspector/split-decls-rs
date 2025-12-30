// Generated macro for get_content_if_ctor_matches (function)
macro_rules! Depcrate_methods_or_then_unwrapget_content_if_ctor_matches {
() => {
// Module: crate::methods::or_then_unwrap
// Provides: {"get_content_if_ctor_matches"}
// Dependencies: {}
fn get_content_if_ctor_matches (cx : & LateContext < '_ > , expr : & Expr < '_ > , item : LangItem) -> Option < Span > { if let ExprKind :: Call (some_expr , [arg]) = expr . kind && some_expr . res (cx) . ctor_parent (cx) . is_lang_item (cx , item) { Some (arg . span . source_callsite ()) } else { None } }
};
}
