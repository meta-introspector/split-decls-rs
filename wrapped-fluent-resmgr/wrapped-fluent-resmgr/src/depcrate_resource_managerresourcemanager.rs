// Generated macro for ResourceManager (struct)
macro_rules! Depcrate_resource_managerResourceManager {
() => {
// Module: crate::resource_manager
// Provides: {"ResourceManager"}
// Dependencies: {}
# [doc = " [`ResourceManager`] provides a standalone solution for managing localization resources which"] # [doc = " can be used by `fluent-fallback` or other higher level bindings."] pub struct ResourceManager { resources : FrozenMap < String , Box < FluentResource > > , path_scheme : String , }
};
}
