// Generated macro for extract_references_map (function)
macro_rules! Depcrate_utilsextract_references_map {
() => {
// Module: crate::utils
// Provides: {"extract_references_map"}
// Dependencies: {}
fn extract_references_map (generics : & Generics) -> HashMap < Ident , HashSet < Ident > > { let mut references = HashMap :: < Ident , HashSet < Ident > > :: default () ; generics . type_params () . for_each (| tp | { SearchSimpleTypeName :: collect_from_type_param (tp) . take () . into_iter () . for_each (| id | { references . entry (id) . or_default () . insert (tp . ident . clone ()) ; }) ; }) ; generics . where_clause . iter () . flat_map (| wc | wc . predicates . iter ()) . filter_map (| wp | wp . maybe_ident () . map (| id | (id , wp))) . for_each (| (ref_ident , wp) | { SearchSimpleTypeName :: collect_from_where_predicate (wp) . take () . into_iter () . for_each (| id | { references . entry (id) . or_default () . insert (ref_ident . clone ()) ; }) ; }) ; references }
};
}
