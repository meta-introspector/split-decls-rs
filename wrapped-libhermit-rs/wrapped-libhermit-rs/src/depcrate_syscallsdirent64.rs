// Generated macro for Dirent64 (struct)
macro_rules! Depcrate_syscallsDirent64 {
() => {
// Module: crate::syscalls
// Provides: {"Dirent64"}
// Dependencies: {}
# [repr (C)] pub struct Dirent64 { # [doc = " 64-bit inode number"] pub d_ino : u64 , # [doc = " Field without meaning. Kept for BW compatibility."] pub d_off : i64 , # [doc = " Size of this dirent"] pub d_reclen : u16 , # [doc = " File type"] pub d_type : fs :: FileType , # [doc = " Filename (null-terminated)"] pub d_name : PhantomData < c_char > , }
};
}
