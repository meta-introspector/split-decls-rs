// Generated macro for impl_9 (impl)
macro_rules! Depcrate_cacheimpl_9 {
() => {
// Module: crate::cache
// Provides: {"impl_9"}
// Dependencies: {}
impl FileIdCache for FileIdMap { fn cached_file_id (& self , path : & Path) -> Option < impl AsRef < FileId > > { self . paths . get (path) } fn add_path (& mut self , path : & Path , recursive_mode : RecursiveMode) { let is_recursive = recursive_mode == RecursiveMode :: Recursive ; for (path , file_id) in WalkDir :: new (path) . follow_links (true) . max_depth (Self :: dir_scan_depth (is_recursive)) . into_iter () . filter_map (| entry | { let path = entry . ok () ? . into_path () ; let file_id = get_file_id (& path) . ok () ? ; Some ((path , file_id)) }) { self . paths . insert (path , file_id) ; } } fn remove_path (& mut self , path : & Path) { self . paths . retain (| p , _ | ! p . starts_with (path)) ; } }
};
}
