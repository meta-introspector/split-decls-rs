// Generated macro for impl_10228 (impl)
macro_rules! Depcrate_trailing_empty_arrayimpl_10228 {
() => {
// Module: crate::trailing_empty_array
// Provides: {"impl_10228"}
// Dependencies: {}
impl < 'tcx > LateLintPass < 'tcx > for TrailingEmptyArray { fn check_item (& mut self , cx : & LateContext < 'tcx > , item : & 'tcx Item < 'tcx >) { if is_struct_with_trailing_zero_sized_array (cx , item) && ! has_repr_attr (cx , item . hir_id ()) && ! is_in_test (cx . tcx , item . hir_id ()) { span_lint_and_help (cx , TRAILING_EMPTY_ARRAY , item . span , "trailing zero-sized array in a struct which is not marked with a `repr` attribute" , None , format ! ("consider annotating `{}` with `#[repr(C)]` or another `repr` attribute" , cx . tcx . def_path_str (item . owner_id)) ,) ; } } }
};
}
