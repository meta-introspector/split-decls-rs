// Generated macro for Blocksize (enum)
macro_rules! Depcrate_fs_fieldsBlocksize {
() => {
// Module: crate::fs::fields
// Provides: {"Blocksize"}
// Dependencies: {}
# [doc = " A file's size of allocated file system blocks."] # [derive (Copy , Clone)] # [cfg (unix)] pub enum Blocksize { # [doc = " This file has the given number of blocks."] Some (u64) , # [doc = " This file isn’t of a type that can take up blocks."] None , }
};
}
