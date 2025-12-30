// Generated macro for RecursiveSize (enum)
macro_rules! Depcrate_fs_recursive_sizeRecursiveSize {
() => {
// Module: crate::fs::recursive_size
// Provides: {"RecursiveSize"}
// Dependencies: {}
# [doc = " Used to represent a the size of a recursive directory traversal.  `None`"] # [doc = " should be used when the file does not represent a directory or the recursive"] # [doc = " size should not be calculated."] # [derive (Copy , Clone , Debug)] pub enum RecursiveSize { # [doc = " Size should not be computed"] None , # [doc = " Size should be computed but has not been computed yet"] Unknown , # [doc = " Size has been computed.  First field is size in bytes and second field"] # [doc = " is size in blocks"] # [cfg_attr (target_family = "windows" , allow (dead_code))] Some (u64 , u64) , }
};
}
