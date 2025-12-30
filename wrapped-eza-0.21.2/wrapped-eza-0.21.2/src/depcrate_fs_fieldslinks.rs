// Generated macro for Links (struct)
macro_rules! Depcrate_fs_fieldsLinks {
() => {
// Module: crate::fs::fields
// Provides: {"Links"}
// Dependencies: {}
# [doc = " A file’s number of hard links on the filesystem."] # [doc = ""] # [doc = " Under Unix, a file can exist on the filesystem only once but appear in"] # [doc = " multiple directories. However, it’s rare (but occasionally useful!) for a"] # [doc = " regular file to have a link count greater than 1, so we highlight the"] # [doc = " block count specifically for this case."] # [allow (unused)] # [derive (Copy , Clone)] pub struct Links { # [doc = " The actual link count."] pub count : nlink_t , # [doc = " Whether this file is a regular file with more than one hard link."] pub multiple : bool , }
};
}
