// Generated macro for is_none_expr (function)
macro_rules! Depcrateis_none_expr {
() => {
// Module: crate
// Provides: {"is_none_expr"}
// Dependencies: {}
# [doc = " Checks is `expr` is `None`"] pub fn is_none_expr (cx : & LateContext < '_ > , expr : & Expr < '_ >) -> bool { expr . res (cx) . ctor_parent (cx) . is_lang_item (cx , OptionNone) }
};
}
