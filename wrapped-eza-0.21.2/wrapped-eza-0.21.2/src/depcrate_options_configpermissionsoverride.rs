// Generated macro for PermissionsOverride (struct)
macro_rules! Depcrate_options_configPermissionsOverride {
() => {
// Module: crate::options::config
// Provides: {"PermissionsOverride"}
// Dependencies: {}
# [rustfmt :: skip] # [derive (Clone , Copy , Eq , Debug , Default , PartialEq , Serialize , Deserialize)] pub struct PermissionsOverride { pub user_read : Option < StyleOverride > , pub user_write : Option < StyleOverride > , pub user_execute_file : Option < StyleOverride > , pub user_execute_other : Option < StyleOverride > , pub group_read : Option < StyleOverride > , pub group_write : Option < StyleOverride > , pub group_execute : Option < StyleOverride > , pub other_read : Option < StyleOverride > , pub other_write : Option < StyleOverride > , pub other_execute : Option < StyleOverride > , pub special_user_file : Option < StyleOverride > , pub special_other : Option < StyleOverride > , pub attribute : Option < StyleOverride > , }
};
}
