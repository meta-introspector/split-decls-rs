// Generated macro for dirent64 (struct)
macro_rules! Depcratedirent64 {
() => {
// Module: crate
// Provides: {"dirent64"}
// Dependencies: {}
# [repr (C)] # [derive (Debug , Clone , Copy)] pub struct dirent64 { # [doc = " 64-bit inode number"] pub d_ino : u64 , # [doc = " 64-bit offset to next structure"] pub d_off : i64 , # [doc = " Size of this dirent"] pub d_reclen : u16 , # [doc = " File type"] pub d_type : u8 , # [doc = " Filename (null-terminated)"] pub d_name : [c_char ; 256] , }
};
}
