// Generated macro for check_and_then_some (function)
macro_rules! Depcrate_methods_bind_instead_of_mapcheck_and_then_some {
() => {
// Module: crate::methods::bind_instead_of_map
// Provides: {"check_and_then_some"}
// Dependencies: {}
pub (super) fn check_and_then_some (cx : & LateContext < '_ > , expr : & hir :: Expr < '_ > , recv : & hir :: Expr < '_ > , arg : & hir :: Expr < '_ > ,) -> bool { BindInsteadOfMap { variant_lang_item : LangItem :: OptionSome , bad_method_name : "and_then" , good_method_name : "map" , } . check (cx , expr , recv , arg) }
};
}
