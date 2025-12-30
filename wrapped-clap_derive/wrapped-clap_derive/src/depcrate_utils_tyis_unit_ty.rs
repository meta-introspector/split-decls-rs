// Generated macro for is_unit_ty (function)
macro_rules! Depcrate_utils_tyis_unit_ty {
() => {
// Module: crate::utils::ty
// Provides: {"is_unit_ty"}
// Dependencies: {}
fn is_unit_ty (ty : & Type) -> bool { if let Type :: Tuple (tuple) = ty { tuple . elems . is_empty () } else { false } }
};
}
