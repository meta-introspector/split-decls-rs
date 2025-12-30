// Generated macro for PermissionsPlus (struct)
macro_rules! Depcrate_fs_fieldsPermissionsPlus {
() => {
// Module: crate::fs::fields
// Provides: {"PermissionsPlus"}
// Dependencies: {}
# [doc = " The three pieces of information that are displayed as a single column in"] # [doc = " the details view. These values are fused together to make the output a"] # [doc = " little more compressed."] # [derive (Copy , Clone)] pub struct PermissionsPlus { # [allow (unused)] pub file_type : Type , # [cfg (unix)] pub permissions : Permissions , # [cfg (windows)] pub attributes : Attributes , # [allow (unused)] pub xattrs : bool , }
};
}
