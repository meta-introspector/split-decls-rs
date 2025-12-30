// Generated macro for check_dirty_clean_annotations (function)
macro_rules! Depcrate_persist_dirty_cleancheck_dirty_clean_annotations {
() => {
// Module: crate::persist::dirty_clean
// Provides: {"check_dirty_clean_annotations"}
// Dependencies: {}
pub (crate) fn check_dirty_clean_annotations (tcx : TyCtxt < '_ >) { if ! tcx . sess . opts . unstable_opts . query_dep_graph { return ; } if ! tcx . features () . rustc_attrs () { return ; } tcx . dep_graph . with_ignore (| | { let mut dirty_clean_visitor = DirtyCleanVisitor { tcx , checked_attrs : Default :: default () } ; let crate_items = tcx . hir_crate_items (()) ; for id in crate_items . free_items () { dirty_clean_visitor . check_item (id . owner_id . def_id) ; } for id in crate_items . trait_items () { dirty_clean_visitor . check_item (id . owner_id . def_id) ; } for id in crate_items . impl_items () { dirty_clean_visitor . check_item (id . owner_id . def_id) ; } for id in crate_items . foreign_items () { dirty_clean_visitor . check_item (id . owner_id . def_id) ; } let mut all_attrs = FindAllAttrs { tcx , found_attrs : vec ! [] } ; tcx . hir_walk_attributes (& mut all_attrs) ; all_attrs . report_unchecked_attrs (dirty_clean_visitor . checked_attrs) ; }) }
};
}
