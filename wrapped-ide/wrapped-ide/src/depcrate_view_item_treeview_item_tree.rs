// Generated macro for view_item_tree (function)
macro_rules! Depcrate_view_item_treeview_item_tree {
() => {
// Module: crate::view_item_tree
// Provides: {"view_item_tree"}
// Dependencies: {}
pub (crate) fn view_item_tree (db : & RootDatabase , file_id : FileId) -> String { let sema = Semantics :: new (db) ; let file_id = sema . attach_first_edition (file_id) . unwrap_or_else (| | EditionedFileId :: current_edition (db , file_id)) ; db . file_item_tree (file_id . into ()) . pretty_print (db , file_id . edition (db)) }
};
}
