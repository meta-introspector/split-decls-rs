// Generated macro for is_struct_with_trailing_zero_sized_array (function)
macro_rules! Depcrate_trailing_empty_arrayis_struct_with_trailing_zero_sized_array {
() => {
// Module: crate::trailing_empty_array
// Provides: {"is_struct_with_trailing_zero_sized_array"}
// Dependencies: {}
fn is_struct_with_trailing_zero_sized_array < 'tcx > (cx : & LateContext < 'tcx > , item : & Item < 'tcx >) -> bool { if let ItemKind :: Struct (_ , _ , data) = & item . kind && let Some (last_field) = data . fields () . last () && let field_ty = cx . tcx . normalize_erasing_regions (cx . typing_env () , cx . tcx . type_of (last_field . def_id) . instantiate_identity () ,) && let ty :: Array (_ , array_len) = * field_ty . kind () && let Some (0) = array_len . try_to_target_usize (cx . tcx) { true } else { false } }
};
}
