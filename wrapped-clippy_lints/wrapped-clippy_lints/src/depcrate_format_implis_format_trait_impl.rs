// Generated macro for is_format_trait_impl (function)
macro_rules! Depcrate_format_implis_format_trait_impl {
() => {
// Module: crate::format_impl
// Provides: {"is_format_trait_impl"}
// Dependencies: {}
fn is_format_trait_impl (cx : & LateContext < '_ > , impl_item : & ImplItem < '_ >) -> Option < FormatTraitNames > { if impl_item . ident . name == sym :: fmt && let ImplItemKind :: Fn (_ , body_id) = impl_item . kind && let Some (Impl { of_trait : Some (of_trait) , .. }) = get_parent_as_impl (cx . tcx , impl_item . hir_id ()) && let Some (did) = of_trait . trait_ref . trait_def_id () && let Some (name) = cx . tcx . get_diagnostic_name (did) && matches ! (name , sym :: Debug | sym :: Display) { let body = cx . tcx . hir_body (body_id) ; let formatter_name = body . params . get (1) . and_then (| param | param . pat . simple_ident ()) . map (| ident | ident . name) ; Some (FormatTraitNames { name , formatter_name }) } else { None } }
};
}
