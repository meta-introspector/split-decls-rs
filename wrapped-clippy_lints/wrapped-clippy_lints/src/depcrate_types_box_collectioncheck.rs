// Generated macro for check (function)
macro_rules! Depcrate_types_box_collectioncheck {
() => {
// Module: crate::types::box_collection
// Provides: {"check"}
// Dependencies: {}
pub (super) fn check (cx : & LateContext < '_ > , hir_ty : & hir :: Ty < '_ > , qpath : & QPath < '_ > , def_id : DefId) -> bool { if Some (def_id) == cx . tcx . lang_items () . owned_box () && let Some (item_type) = get_std_collection (cx , qpath) { let generic = match item_type { sym :: String => "" , _ => "<..>" , } ; let box_content = format ! ("{item_type}{generic}") ; span_lint_and_help (cx , BOX_COLLECTION , hir_ty . span , format ! ("you seem to be trying to use `Box<{box_content}>`. Consider using just `{box_content}`") , None , format ! ("`{box_content}` is already on the heap, `Box<{box_content}>` makes an extra allocation") ,) ; true } else { false } }
};
}
