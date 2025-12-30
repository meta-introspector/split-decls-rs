// Generated macro for is_zero_sized_ty (function)
macro_rules! Depcrate_transmute_transmute_undefined_repris_zero_sized_ty {
() => {
// Module: crate::transmute::transmute_undefined_repr
// Provides: {"is_zero_sized_ty"}
// Dependencies: {}
fn is_zero_sized_ty < 'tcx > (cx : & LateContext < 'tcx > , ty : Ty < 'tcx >) -> bool { if let Ok (ty) = cx . tcx . try_normalize_erasing_regions (cx . typing_env () , ty) && let Ok (layout) = cx . tcx . layout_of (cx . typing_env () . as_query_input (ty)) { layout . layout . size () . bytes () == 0 } else { false } }
};
}
