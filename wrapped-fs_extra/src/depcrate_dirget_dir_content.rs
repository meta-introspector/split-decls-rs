// Generated macro for get_dir_content (function)
macro_rules! Depcrate_dirget_dir_content {
() => {
// Module: crate::dir
// Provides: {"get_dir_content"}
// Dependencies: {}
# [doc = " Return DirContent which contains information about directory:"] # [doc = ""] # [doc = " * Size of the directory in bytes."] # [doc = " * List of source paths of files in the directory (files inside subdirectories included too)."] # [doc = " * List of source paths of all directories and subdirectories."] # [doc = ""] # [doc = " # Errors"] # [doc = ""] # [doc = " This function will return an error in the following situations, but is not limited to just"] # [doc = " these cases:"] # [doc = ""] # [doc = " * This `path` directory does not exist."] # [doc = " * Invalid `path`."] # [doc = " * The current process does not have the permission to access `path`."] # [doc = ""] # [doc = " # Examples"] # [doc = " ```rust,ignore"] # [doc = " extern crate fs_extra;"] # [doc = " use fs_extra::dir::get_dir_content;"] # [doc = ""] # [doc = " let dir_content = get_dir_content(\"dir\")?;"] # [doc = " for directory in dir_content.directories {"] # [doc = "     println!(\"{}\", directory); // print directory path"] # [doc = " }"] # [doc = " ```"] # [doc = ""] pub fn get_dir_content < P > (path : P) -> Result < DirContent > where P : AsRef < Path > , { let options = DirOptions :: new () ; get_dir_content2 (path , & options) }
};
}
