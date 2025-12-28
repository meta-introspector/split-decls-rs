macro_rules! macro_38 {
    () => {
        # [cfg (all (target_os = "linux" , target_env = "gnu"))] # [cfg (feature = "fs")] libc_bitflags ! { # [doc = " Flags for use with [`renameat2`]."] # [cfg_attr (docsrs , doc (cfg (feature = "fs")))] pub struct RenameFlags : u32 { # [doc = " Atomically exchange `old_path` and `new_path`."] RENAME_EXCHANGE ; # [doc = " Don't overwrite `new_path` of the rename.  Return an error if `new_path` already"] # [doc = " exists."] RENAME_NOREPLACE ; # [doc = " creates a \"whiteout\" object at the source of the rename at the same time as performing"] # [doc = " the rename."] # [doc = ""] # [doc = " This operation makes sense only for overlay/union filesystem implementations."] RENAME_WHITEOUT ; } }
    };
}

macro_38!()