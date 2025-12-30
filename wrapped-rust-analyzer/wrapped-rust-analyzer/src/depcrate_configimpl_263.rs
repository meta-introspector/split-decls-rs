// Generated macro for impl_263 (impl)
macro_rules! Depcrate_configimpl_263 {
() => {
// Module: crate::config
// Provides: {"impl_263"}
// Dependencies: {}
impl fmt :: Debug for Config { fn fmt (& self , f : & mut fmt :: Formatter < '_ >) -> fmt :: Result { f . debug_struct ("Config") . field ("discovered_projects_from_filesystem" , & self . discovered_projects_from_filesystem) . field ("discovered_projects_from_command" , & self . discovered_projects_from_command) . field ("workspace_roots" , & self . workspace_roots) . field ("caps" , & self . caps) . field ("root_path" , & self . root_path) . field ("snippets" , & self . snippets) . field ("client_info" , & self . client_info) . field ("client_config" , & self . client_config) . field ("user_config" , & self . user_config) . field ("ratoml_file" , & self . ratoml_file) . field ("source_root_parent_map" , & self . source_root_parent_map) . field ("validation_errors" , & self . validation_errors) . field ("detached_files" , & self . detached_files) . finish () } }
};
}
