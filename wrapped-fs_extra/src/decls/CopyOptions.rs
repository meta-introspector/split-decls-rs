macro_rules! CopyOptions {
    () => {
        # [doc = " Options and flags which can be used to configure how a file will be copied or moved."] # [derive (Clone)] pub struct CopyOptions { # [doc = " Overwrite existing files if true (default: false)."] pub overwrite : bool , # [doc = " Skip existing files if true (default: false)."] pub skip_exist : bool , # [doc = " Buffer size that specifies the amount of bytes to be moved or copied before the progress handler is called. This only affects functions with progress handlers. (default: 64000)"] pub buffer_size : usize , # [doc = " Recursively copy a directory with a new name or place it inside the destination (default: false, same behaviors as cp -r on Unix)"] pub copy_inside : bool , # [doc = " Copy only contents without a creating a new folder in the destination folder (default: false)."] pub content_only : bool , # [doc = " Sets levels reading. Set 0 for read all directory folder (default: 0)."] # [doc = ""] # [doc = " Warning: Work only for copy operations!"] pub depth : u64 , }
    };
}

CopyOptions!()