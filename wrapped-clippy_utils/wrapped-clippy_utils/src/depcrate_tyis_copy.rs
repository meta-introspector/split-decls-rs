// Generated macro for is_copy (function)
macro_rules! Depcrate_tyis_copy {
() => {
// Module: crate::ty
// Provides: {"is_copy"}
// Dependencies: {}
# [doc = " Checks if the given type implements copy."] pub fn is_copy < 'tcx > (cx : & LateContext < 'tcx > , ty : Ty < 'tcx >) -> bool { cx . type_is_copy_modulo_regions (ty) }
};
}
