// Generated macro for FileKindsOverride (struct)
macro_rules! Depcrate_options_configFileKindsOverride {
() => {
// Module: crate::options::config
// Provides: {"FileKindsOverride"}
// Dependencies: {}
# [rustfmt :: skip] # [derive (Clone , Eq , Copy , Debug , PartialEq , Serialize , Deserialize)] pub struct FileKindsOverride { pub normal : Option < StyleOverride > , pub directory : Option < StyleOverride > , pub symlink : Option < StyleOverride > , pub pipe : Option < StyleOverride > , pub block_device : Option < StyleOverride > , pub char_device : Option < StyleOverride > , pub socket : Option < StyleOverride > , pub special : Option < StyleOverride > , pub executable : Option < StyleOverride > , pub mount_point : Option < StyleOverride > , }
};
}
