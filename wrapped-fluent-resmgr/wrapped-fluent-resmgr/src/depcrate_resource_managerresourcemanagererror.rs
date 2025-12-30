// Generated macro for ResourceManagerError (enum)
macro_rules! Depcrate_resource_managerResourceManagerError {
() => {
// Module: crate::resource_manager
// Provides: {"ResourceManagerError"}
// Dependencies: {}
# [doc = " Errors generated during the process of retrieving the localization resources"] # [derive (Debug , Error)] pub enum ResourceManagerError { # [doc = " Error while reading the resource file"] # [error ("{0}")] Io (# [from] std :: io :: Error) , # [doc = " Error while trying to add a resource to the bundle"] # [error ("{0}")] Fluent (# [from] fluent_bundle :: FluentError) , }
};
}
