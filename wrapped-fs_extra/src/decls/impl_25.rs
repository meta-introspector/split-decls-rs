macro_rules! deps {
    () => {
        CopyOptions!();
    };
}

macro_rules! impl_25 {
    () => {
        deps!();
        impl CopyOptions { # [doc = " Initialize struct CopyOptions with default value."] # [doc = ""] # [doc = " ```rust,ignore"] # [doc = " overwrite: false"] # [doc = ""] # [doc = " skip_exist: false"] # [doc = ""] # [doc = " buffer_size: 64000 // 64kb"] # [doc = ""] # [doc = " copy_inside: false"] # [doc = " ```"] pub fn new () -> CopyOptions { CopyOptions { overwrite : false , skip_exist : false , buffer_size : 64000 , copy_inside : false , content_only : false , depth : 0 , } } # [doc = " Overwrite existing files if true."] pub fn overwrite (mut self , overwrite : bool) -> Self { self . overwrite = overwrite ; self } # [doc = " Skip existing files if true."] pub fn skip_exist (mut self , skip_exist : bool) -> Self { self . skip_exist = skip_exist ; self } # [doc = " Buffer size that specifies the amount of bytes to be moved or copied before the progress handler is called. This only affects functions with progress handlers."] pub fn buffer_size (mut self , buffer_size : usize) -> Self { self . buffer_size = buffer_size ; self } # [doc = " Recursively copy a directory with a new name or place it inside the destination (default: false, same behaviors as cp -r on Unix)"] pub fn copy_inside (mut self , copy_inside : bool) -> Self { self . copy_inside = copy_inside ; self } # [doc = " Copy only contents without a creating a new folder in the destination folder."] pub fn content_only (mut self , content_only : bool) -> Self { self . content_only = content_only ; self } # [doc = " Sets levels reading. Set 0 for read all directory folder"] pub fn depth (mut self , depth : u64) -> Self { self . depth = depth ; self } }
    };
}

impl_25!();