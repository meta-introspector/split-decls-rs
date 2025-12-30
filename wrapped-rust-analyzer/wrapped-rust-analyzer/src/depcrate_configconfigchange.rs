// Generated macro for ConfigChange (struct)
macro_rules! Depcrate_configConfigChange {
() => {
// Module: crate::config
// Provides: {"ConfigChange"}
// Dependencies: {}
# [derive (Default , Debug)] pub struct ConfigChange { user_config_change : Option < Arc < str > > , client_config_change : Option < serde_json :: Value > , ratoml_file_change : Option < FxHashMap < SourceRootId , (RatomlFileKind , VfsPath , Option < Arc < str > >) > > , source_map_change : Option < Arc < FxHashMap < SourceRootId , SourceRootId > > > , }
};
}
