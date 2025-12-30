// Generated macro for set_permissions (function)
macro_rules! Depcrate_fsset_permissions {
() => {
// Module: crate::fs
// Provides: {"set_permissions"}
// Dependencies: {}
# [doc = " A wrapper around [`std::fs::set_permissions`] which includes the file path in the panic message."] # [track_caller] pub fn set_permissions < P : AsRef < Path > > (path : P , perm : std :: fs :: Permissions) { std :: fs :: set_permissions (path . as_ref () , perm) . expect (& format ! ("the file's permissions in path \"{}\" could not be changed" , path . as_ref () . display ())) ; }
};
}
