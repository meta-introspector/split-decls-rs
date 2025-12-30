// Generated macro for view_crate_graph (function)
macro_rules! Depcrate_view_crate_graphview_crate_graph {
() => {
// Module: crate::view_crate_graph
// Provides: {"view_crate_graph"}
// Dependencies: {}
pub (crate) fn view_crate_graph (db : & RootDatabase , full : bool) -> Result < String , String > { let all_crates = db . all_crates () ; let crates_to_render = all_crates . iter () . copied () . map (| krate | (krate , (krate . data (db) , krate . extra_data (db)))) . filter (| (_ , (crate_data , _)) | { if full { true } else { let root_id = db . file_source_root (crate_data . root_file_id) . source_root_id (db) ; ! db . source_root (root_id) . source_root (db) . is_library } }) . collect () ; let graph = DotCrateGraph { crates_to_render } ; let mut dot = Vec :: new () ; dot :: render (& graph , & mut dot) . unwrap () ; Ok (String :: from_utf8 (dot) . unwrap ()) }
};
}
