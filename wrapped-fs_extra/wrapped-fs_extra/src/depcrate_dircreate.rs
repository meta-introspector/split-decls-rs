// Generated macro for create (function)
macro_rules! Depcrate_dircreate {
() => {
// Module: crate::dir
// Provides: {"create"}
// Dependencies: {}
# [doc = " Creates a new, empty directory at the provided path."] # [doc = ""] # [doc = " This function takes to arguments:"] # [doc = ""] # [doc = " * `path` - Path to new directory."] # [doc = ""] # [doc = " * `erase` - If set true and folder exist, then folder will be erased."] # [doc = ""] # [doc = " #Errors"] # [doc = ""] # [doc = " This function will return an error in the following situations,"] # [doc = " but is not limited to just these cases:"] # [doc = ""] # [doc = " * User lacks permissions to create directory at `path`."] # [doc = ""] # [doc = " * `path` already exists if `erase` set false."] # [doc = ""] # [doc = " #Examples"] # [doc = ""] # [doc = " ```rust,ignore"] # [doc = " extern crate fs_extra;"] # [doc = " use fs_extra::dir::create;"] # [doc = ""] # [doc = " create(\"dir\", false); // create directory"] # [doc = " ```"] pub fn create < P > (path : P , erase : bool) -> Result < () > where P : AsRef < Path > , { if erase && path . as_ref () . exists () { remove (& path) ? ; } Ok (create_dir (& path) ?) }
};
}
