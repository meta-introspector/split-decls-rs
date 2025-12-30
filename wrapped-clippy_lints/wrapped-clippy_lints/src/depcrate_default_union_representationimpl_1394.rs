// Generated macro for impl_1394 (impl)
macro_rules! Depcrate_default_union_representationimpl_1394 {
() => {
// Module: crate::default_union_representation
// Provides: {"impl_1394"}
// Dependencies: {}
impl < 'tcx > LateLintPass < 'tcx > for DefaultUnionRepresentation { fn check_item (& mut self , cx : & LateContext < 'tcx > , item : & 'tcx Item < 'tcx >) { if ! item . span . from_expansion () && is_union_with_two_non_zst_fields (cx , item) && ! has_c_repr_attr (cx , item . hir_id ()) { # [expect (clippy :: collapsible_span_lint_calls , reason = "rust-clippy#7797")] span_lint_and_then (cx , DEFAULT_UNION_REPRESENTATION , item . span , "this union has the default representation" , | diag | { diag . help (format ! ("consider annotating `{}` with `#[repr(C)]` to explicitly specify memory layout" , cx . tcx . def_path_str (item . owner_id))) ; } ,) ; } } }
};
}
