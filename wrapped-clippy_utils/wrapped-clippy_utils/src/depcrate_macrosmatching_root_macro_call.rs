// Generated macro for matching_root_macro_call (function)
macro_rules! Depcrate_macrosmatching_root_macro_call {
() => {
// Module: crate::macros
// Provides: {"matching_root_macro_call"}
// Dependencies: {}
# [doc = " A combination of [`root_macro_call`] and"] # [doc = " [`is_diagnostic_item`](rustc_middle::ty::TyCtxt::is_diagnostic_item) that returns a `MacroCall`"] # [doc = " at the root expansion if only it matches the given name."] pub fn matching_root_macro_call (cx : & LateContext < '_ > , span : Span , name : Symbol) -> Option < MacroCall > { root_macro_call (span) . filter (| mc | cx . tcx . is_diagnostic_item (name , mc . def_id)) }
};
}
