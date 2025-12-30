// Generated macro for Permissions (struct)
macro_rules! Depcrate_fs_fieldsPermissions {
() => {
// Module: crate::fs::fields
// Provides: {"Permissions"}
// Dependencies: {}
# [doc = " The file’s Unix permission bitfield, with one entry per bit."] # [derive (Copy , Clone)] # [rustfmt :: skip] pub struct Permissions { pub user_read : bool , pub user_write : bool , pub user_execute : bool , pub group_read : bool , pub group_write : bool , pub group_execute : bool , pub other_read : bool , pub other_write : bool , pub other_execute : bool , pub sticky : bool , pub setgid : bool , pub setuid : bool , }
};
}
