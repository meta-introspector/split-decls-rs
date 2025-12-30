// Generated macro for is_none_pattern (function)
macro_rules! Depcrateis_none_pattern {
() => {
// Module: crate
// Provides: {"is_none_pattern"}
// Dependencies: {}
# [doc = " Checks if the `pat` is `None`."] pub fn is_none_pattern (cx : & LateContext < '_ > , pat : & Pat < '_ >) -> bool { matches ! (pat . kind , PatKind :: Expr (PatExpr { kind : PatExprKind :: Path (qpath) , .. }) if cx . qpath_res (qpath , pat . hir_id) . ctor_parent (cx) . is_lang_item (cx , OptionNone)) }
};
}
