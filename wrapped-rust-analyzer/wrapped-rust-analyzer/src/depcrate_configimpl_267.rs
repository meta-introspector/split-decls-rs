// Generated macro for impl_267 (impl)
macro_rules! Depcrate_configimpl_267 {
() => {
// Module: crate::config
// Provides: {"impl_267"}
// Dependencies: {}
impl ConfigChange { pub fn change_ratoml (& mut self , source_root : SourceRootId , vfs_path : VfsPath , content : Option < Arc < str > > ,) -> Option < (RatomlFileKind , VfsPath , Option < Arc < str > >) > { self . ratoml_file_change . get_or_insert_with (Default :: default) . insert (source_root , (RatomlFileKind :: Crate , vfs_path , content)) } pub fn change_user_config (& mut self , content : Option < Arc < str > >) { assert ! (self . user_config_change . is_none ()) ; self . user_config_change = content ; } pub fn change_workspace_ratoml (& mut self , source_root : SourceRootId , vfs_path : VfsPath , content : Option < Arc < str > > ,) -> Option < (RatomlFileKind , VfsPath , Option < Arc < str > >) > { self . ratoml_file_change . get_or_insert_with (Default :: default) . insert (source_root , (RatomlFileKind :: Workspace , vfs_path , content)) } pub fn change_client_config (& mut self , change : serde_json :: Value) { self . client_config_change = Some (change) ; } pub fn change_source_root_parent_map (& mut self , source_root_map : Arc < FxHashMap < SourceRootId , SourceRootId > > ,) { assert ! (self . source_map_change . is_none ()) ; self . source_map_change = Some (source_root_map) ; } }
};
}
