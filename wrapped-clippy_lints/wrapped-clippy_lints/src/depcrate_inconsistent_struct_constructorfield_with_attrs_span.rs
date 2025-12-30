// Generated macro for field_with_attrs_span (function)
macro_rules! Depcrate_inconsistent_struct_constructorfield_with_attrs_span {
() => {
// Module: crate::inconsistent_struct_constructor
// Provides: {"field_with_attrs_span"}
// Dependencies: {}
fn field_with_attrs_span (tcx : TyCtxt < '_ > , field : & hir :: ExprField < '_ >) -> Span { if let Some (attr) = tcx . hir_attrs (field . hir_id) . first () { field . span . with_lo (attr . span () . lo ()) } else { field . span } }
};
}
