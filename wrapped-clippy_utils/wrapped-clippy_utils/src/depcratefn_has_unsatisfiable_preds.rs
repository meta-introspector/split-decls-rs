// Generated macro for fn_has_unsatisfiable_preds (function)
macro_rules! Depcratefn_has_unsatisfiable_preds {
() => {
// Module: crate
// Provides: {"fn_has_unsatisfiable_preds"}
// Dependencies: {}
# [doc = " Check if it's even possible to satisfy the `where` clause for the item."] # [doc = ""] # [doc = " `trivial_bounds` feature allows functions with unsatisfiable bounds, for example:"] # [doc = ""] # [doc = " ```ignore"] # [doc = " fn foo() where i32: Iterator {"] # [doc = "     for _ in 2i32 {}"] # [doc = " }"] # [doc = " ```"] pub fn fn_has_unsatisfiable_preds (cx : & LateContext < '_ > , did : DefId) -> bool { use rustc_trait_selection :: traits ; let predicates = cx . tcx . predicates_of (did) . predicates . iter () . filter_map (| (p , _) | if p . is_global () { Some (* p) } else { None }) ; traits :: impossible_predicates (cx . tcx , traits :: elaborate (cx . tcx , predicates) . collect :: < Vec < _ > > ()) }
};
}
