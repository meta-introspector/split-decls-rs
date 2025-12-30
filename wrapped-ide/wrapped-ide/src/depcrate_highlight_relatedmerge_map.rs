// Generated macro for merge_map (function)
macro_rules! Depcrate_highlight_relatedmerge_map {
() => {
// Module: crate::highlight_related
// Provides: {"merge_map"}
// Dependencies: {}
fn merge_map (res : & mut HighlightMap , new : Option < HighlightMap >) { let Some (new) = new else { return ; } ; new . into_iter () . for_each (| (file_id , ranges) | { res . entry (file_id) . or_default () . extend (ranges) ; }) ; }
};
}
