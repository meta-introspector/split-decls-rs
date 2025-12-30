// Generated macro for get_const_name_and_ty_name (function)
macro_rules! Depcrate_casts_confusing_method_to_numeric_castget_const_name_and_ty_name {
() => {
// Module: crate::casts::confusing_method_to_numeric_cast
// Provides: {"get_const_name_and_ty_name"}
// Dependencies: {}
fn get_const_name_and_ty_name (cx : & LateContext < '_ > , method_name : Symbol , method_def_id : DefId , generics : & [GenericArg < '_ >] ,) -> Option < (& 'static str , & 'static str) > { let diagnostic_name = cx . tcx . get_diagnostic_name (method_def_id) ; let ty_name = if diagnostic_name . is_some_and (| diag | diag == sym :: cmp_ord_min || diag == sym :: cmp_ord_max) { if let [ty] = generics && let Some (ty) = ty . as_type () { get_primitive_ty_name (ty) ? } else { return None ; } } else if let Some (impl_id) = cx . tcx . impl_of_assoc (method_def_id) && let Some (ty_name) = get_primitive_ty_name (cx . tcx . type_of (impl_id) . instantiate_identity ()) && matches ! (method_name , sym :: min | sym :: max | sym :: minimum | sym :: maximum | sym :: min_value | sym :: max_value) { ty_name } else { return None ; } ; let const_name = if matches ! (method_name , sym :: max | sym :: maximum | sym :: max_value) { "MAX" } else { "MIN" } ; Some ((const_name , ty_name)) }
};
}
