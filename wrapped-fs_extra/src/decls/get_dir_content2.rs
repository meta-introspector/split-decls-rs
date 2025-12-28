macro_rules! deps {
    () => {
        DirOptions!();
        Result!();
        DirContent!();
    };
}

macro_rules! get_dir_content2 {
    () => {
        deps!();
        # [doc = " Return DirContent which contains information about directory:"] # [doc = ""] # [doc = " * Size directory."] # [doc = " * List all files source directory(files subdirectories  included too)."] # [doc = " * List all directory and subdirectories source path."] # [doc = ""] # [doc = " # Errors"] # [doc = ""] # [doc = " This function will return an error in the following situations, but is not limited to just"] # [doc = " these cases:"] # [doc = ""] # [doc = " * This `path` directory does not exist."] # [doc = " * Invalid `path`."] # [doc = " * The current process does not have the permission to access `path`."] # [doc = ""] # [doc = " # Examples"] # [doc = " ```rust,ignore"] # [doc = " extern crate fs_extra;"] # [doc = " use fs_extra::dir::{DirOptions, get_dir_content2};"] # [doc = ""] # [doc = " let mut options = DirOptions::new();"] # [doc = " options.depth = 3; // Get 3 levels of folder."] # [doc = " let dir_content = get_dir_content2(\"dir\", &options)?;"] # [doc = " for directory in dir_content.directories {"] # [doc = "     println!(\"{}\", directory); // print directory path"] # [doc = " }"] # [doc = " ```"] # [doc = ""] pub fn get_dir_content2 < P > (path : P , options : & DirOptions) -> Result < DirContent > where P : AsRef < Path > , { let mut depth = 0 ; if options . depth != 0 { depth = options . depth + 1 ; } _get_dir_content (path , depth) }
    };
}

get_dir_content2!();