// Generated macro for include_attrs_in_span (function)
macro_rules! Depcrate_undocumented_unsafe_blocksinclude_attrs_in_span {
() => {
// Module: crate::undocumented_unsafe_blocks
// Provides: {"include_attrs_in_span"}
// Dependencies: {}
fn include_attrs_in_span (cx : & LateContext < '_ > , hir_id : HirId , span : Span) -> Span { span . to (cx . tcx . hir_attrs (hir_id) . iter () . fold (span , | acc , attr | { if attr . is_doc_comment () { return acc ; } acc . to (attr . span ()) })) }
};
}
