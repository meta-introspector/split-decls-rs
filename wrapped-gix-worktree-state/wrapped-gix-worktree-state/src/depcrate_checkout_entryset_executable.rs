// Generated macro for set_executable (function)
macro_rules! Depcrate_checkout_entryset_executable {
() => {
// Module: crate::checkout::entry
// Provides: {"set_executable"}
// Dependencies: {}
# [doc = " Use `fstat` and `fchmod` on a file descriptor to make a regular file executable."] # [doc = ""] # [doc = " See `let_readers_execute` for the exact details of how the mode is transformed."] # [cfg (unix)] fn set_executable (file : & std :: fs :: File) -> Result < () , std :: io :: Error > { use std :: os :: unix :: fs :: { MetadataExt , PermissionsExt } ; let old_mode = file . metadata () ? . mode () ; let new_mode = let_readers_execute (old_mode) ; file . set_permissions (std :: fs :: Permissions :: from_mode (new_mode)) ? ; Ok (()) }
};
}
