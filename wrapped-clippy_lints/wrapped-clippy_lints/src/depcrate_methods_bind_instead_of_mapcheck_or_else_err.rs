// Generated macro for check_or_else_err (function)
macro_rules! Depcrate_methods_bind_instead_of_mapcheck_or_else_err {
() => {
// Module: crate::methods::bind_instead_of_map
// Provides: {"check_or_else_err"}
// Dependencies: {}
pub (super) fn check_or_else_err (cx : & LateContext < '_ > , expr : & hir :: Expr < '_ > , recv : & hir :: Expr < '_ > , arg : & hir :: Expr < '_ > ,) -> bool { BindInsteadOfMap { variant_lang_item : LangItem :: ResultErr , bad_method_name : "or_else" , good_method_name : "map_err" , } . check (cx , expr , recv , arg) }
};
}
