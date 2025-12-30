// Generated macro for get_iterator_item_ty (function)
macro_rules! Depcrate_tyget_iterator_item_ty {
() => {
// Module: crate::ty
// Provides: {"get_iterator_item_ty"}
// Dependencies: {}
# [doc = " Resolves `<T as Iterator>::Item` for `T`"] # [doc = " Do not invoke without first verifying that the type implements `Iterator`"] pub fn get_iterator_item_ty < 'tcx > (cx : & LateContext < 'tcx > , ty : Ty < 'tcx >) -> Option < Ty < 'tcx > > { cx . tcx . get_diagnostic_item (sym :: Iterator) . and_then (| iter_did | cx . get_associated_type (ty , iter_did , sym :: Item)) }
};
}
