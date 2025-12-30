// Generated macro for has_debug_impl (function)
macro_rules! Depcrate_tyhas_debug_impl {
() => {
// Module: crate::ty
// Provides: {"has_debug_impl"}
// Dependencies: {}
# [doc = " This checks whether a given type is known to implement Debug."] pub fn has_debug_impl < 'tcx > (cx : & LateContext < 'tcx > , ty : Ty < 'tcx >) -> bool { cx . tcx . get_diagnostic_item (sym :: Debug) . is_some_and (| debug | implements_trait (cx , ty , debug , & [])) }
};
}
