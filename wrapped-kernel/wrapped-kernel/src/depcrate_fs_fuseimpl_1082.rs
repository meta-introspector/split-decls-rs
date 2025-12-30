// Generated macro for impl_1082 (impl)
macro_rules! Depcrate_fs_fuseimpl_1082 {
() => {
// Module: crate::fs::fuse
// Provides: {"impl_1082"}
// Dependencies: {}
impl From < fuse_attr > for FileAttr { fn from (attr : fuse_attr) -> FileAttr { FileAttr { st_ino : attr . ino , st_nlink : attr . nlink . into () , st_mode : AccessPermission :: from_bits_retain (attr . mode) , st_uid : attr . uid , st_gid : attr . gid , st_rdev : attr . rdev . into () , st_size : attr . size . try_into () . unwrap () , st_blksize : attr . blksize . into () , st_blocks : attr . blocks . try_into () . unwrap () , st_atim : timespec { tv_sec : attr . atime as time_t , tv_nsec : attr . atimensec as i32 , } , st_mtim : timespec { tv_sec : attr . mtime as time_t , tv_nsec : attr . mtimensec as i32 , } , st_ctim : timespec { tv_sec : attr . ctime as time_t , tv_nsec : attr . ctimensec as i32 , } , .. Default :: default () } } }
};
}
