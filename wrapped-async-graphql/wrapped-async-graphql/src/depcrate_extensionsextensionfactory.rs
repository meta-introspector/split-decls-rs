// Generated macro for ExtensionFactory (trait)
macro_rules! Depcrate_extensionsExtensionFactory {
() => {
// Module: crate::extensions
// Provides: {"ExtensionFactory"}
// Dependencies: {}
# [doc = " Extension factory"] # [doc = ""] # [doc = " Used to create an extension instance."] pub trait ExtensionFactory : Send + Sync + 'static { # [doc = " Create an extended instance."] fn create (& self) -> Arc < dyn Extension > ; }
};
}
