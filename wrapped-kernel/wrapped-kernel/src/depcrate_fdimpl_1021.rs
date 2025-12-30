// Generated macro for impl_1021 (impl)
macro_rules! Depcrate_fdimpl_1021 {
() => {
// Module: crate::fd
// Provides: {"impl_1021"}
// Dependencies: {}
impl AccessOption { # [doc = " Verifies if the current access options are all valid for the provided file access permissions"] pub fn can_access (& self , access_permissions : AccessPermission) -> bool { if self . contains (AccessOption :: R_OK) && ! access_permissions . contains (AccessPermission :: S_IRUSR) && ! access_permissions . contains (AccessPermission :: S_IRGRP) && ! access_permissions . contains (AccessPermission :: S_IROTH) { return false ; } if self . contains (AccessOption :: W_OK) && ! access_permissions . contains (AccessPermission :: S_IWUSR) && ! access_permissions . contains (AccessPermission :: S_IWGRP) && ! access_permissions . contains (AccessPermission :: S_IWOTH) { return false ; } if self . contains (AccessOption :: X_OK) && ! access_permissions . contains (AccessPermission :: S_IXUSR) && ! access_permissions . contains (AccessPermission :: S_IXGRP) && ! access_permissions . contains (AccessPermission :: S_IXOTH) { return false ; } true } }
};
}
