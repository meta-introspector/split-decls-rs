// Generated macro for impl_2329 (impl)
macro_rules! Depcrate_format_implimpl_2329 {
() => {
// Module: crate::format_impl
// Provides: {"impl_2329"}
// Dependencies: {}
impl < 'tcx > LateLintPass < 'tcx > for FormatImpl { fn check_impl_item (& mut self , cx : & LateContext < '_ > , impl_item : & ImplItem < '_ >) { self . format_trait_impl = is_format_trait_impl (cx , impl_item) ; } fn check_impl_item_post (& mut self , cx : & LateContext < '_ > , impl_item : & ImplItem < '_ >) { if is_format_trait_impl (cx , impl_item) . is_some () { self . format_trait_impl = None ; } } fn check_expr (& mut self , cx : & LateContext < 'tcx > , expr : & 'tcx Expr < '_ >) { if let Some (format_trait_impl) = self . format_trait_impl { let linter = FormatImplExpr { cx , format_args : & self . format_args , expr , format_trait_impl , } ; linter . check_to_string_in_display () ; linter . check_self_in_format_args () ; linter . check_print_in_format_impl () ; } } }
};
}
