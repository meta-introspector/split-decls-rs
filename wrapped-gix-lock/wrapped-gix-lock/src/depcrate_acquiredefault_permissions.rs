// Generated macro for default_permissions (function)
macro_rules! Depcrate_acquiredefault_permissions {
() => {
// Module: crate::acquire
// Provides: {"default_permissions"}
// Dependencies: {}
fn default_permissions () -> Option < std :: fs :: Permissions > { # [cfg (unix)] { use std :: os :: unix :: fs :: PermissionsExt ; Some (std :: fs :: Permissions :: from_mode (0o666)) } # [cfg (not (unix))] { None } }
};
}
