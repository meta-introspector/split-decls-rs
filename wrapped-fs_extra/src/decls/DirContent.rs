macro_rules! DirContent {
    () => {
        # [doc = " A structure which include information about directory"] pub struct DirContent { # [doc = " Directory size in bytes."] pub dir_size : u64 , # [doc = " List all files directory and sub directories."] pub files : Vec < String > , # [doc = " List all folders and sub folders directory."] pub directories : Vec < String > , }
    };
}

DirContent!()