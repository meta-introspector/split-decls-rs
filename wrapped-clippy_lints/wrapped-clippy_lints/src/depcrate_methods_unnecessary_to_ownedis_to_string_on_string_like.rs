// Generated macro for is_to_string_on_string_like (function)
macro_rules! Depcrate_methods_unnecessary_to_ownedis_to_string_on_string_like {
() => {
// Module: crate::methods::unnecessary_to_owned
// Provides: {"is_to_string_on_string_like"}
// Dependencies: {}
# [doc = " Returns true if the named method is `ToString::to_string` and it's called on a type that"] # [doc = " is string-like i.e. implements `AsRef<str>` or `Deref<Target = str>`."] fn is_to_string_on_string_like < 'a > (cx : & LateContext < '_ > , call_expr : & 'a Expr < 'a > , method_name : Symbol , method_parent_id : DefId ,) -> bool { if method_name != sym :: to_string || ! method_parent_id . is_diag_item (cx , sym :: ToString) { return false ; } if let Some (args) = cx . typeck_results () . node_args_opt (call_expr . hir_id) && let [generic_arg] = args . as_slice () && let GenericArgKind :: Type (ty) = generic_arg . kind () && let Some (deref_trait_id) = cx . tcx . get_diagnostic_item (sym :: Deref) && let Some (as_ref_trait_id) = cx . tcx . get_diagnostic_item (sym :: AsRef) && (cx . get_associated_type (ty , deref_trait_id , sym :: Target) == Some (cx . tcx . types . str_) || implements_trait (cx , ty , as_ref_trait_id , & [cx . tcx . types . str_ . into ()])) { true } else { false } }
};
}
