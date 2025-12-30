// Generated macro for FileAttr (struct)
macro_rules! Depcrate_fsFileAttr {
() => {
// Module: crate::fs
// Provides: {"FileAttr"}
// Dependencies: {}
# [repr (C)] # [derive (Debug , Default , Copy , Clone)] pub struct FileAttr { pub st_dev : u64 , pub st_ino : u64 , pub st_nlink : u64 , # [doc = " access permissions"] pub st_mode : AccessPermission , # [doc = " user id"] pub st_uid : u32 , # [doc = " group id"] pub st_gid : u32 , # [doc = " device id"] pub st_rdev : u64 , # [doc = " size in bytes"] pub st_size : i64 , # [doc = " block size"] pub st_blksize : i64 , # [doc = " size in blocks"] pub st_blocks : i64 , # [doc = " time of last access"] pub st_atim : timespec , # [doc = " time of last modification"] pub st_mtim : timespec , # [doc = " time of last status change"] pub st_ctim : timespec , }
};
}
