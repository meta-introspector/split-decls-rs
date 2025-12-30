// Generated macro for original_frange (function)
macro_rules! Depcrate_highlight_relatedoriginal_frange {
() => {
// Module: crate::highlight_related
// Provides: {"original_frange"}
// Dependencies: {}
fn original_frange (db : & dyn db :: ExpandDatabase , file_id : HirFileId , text_range : Option < TextRange > ,) -> Option < FileRange > { InFile :: new (file_id , text_range ?) . original_node_file_range_opt (db) . map (| (frange , _) | frange) }
};
}
