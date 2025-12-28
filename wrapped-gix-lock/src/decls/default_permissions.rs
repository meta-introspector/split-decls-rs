macro_rules! default_permissions {
    () => {
        fn default_permissions () -> Option < std :: fs :: Permissions > { # [cfg (unix)] { use std :: os :: unix :: fs :: PermissionsExt ; Some (std :: fs :: Permissions :: from_mode (0o666)) } # [cfg (not (unix))] { None } }
    };
}

default_permissions!();