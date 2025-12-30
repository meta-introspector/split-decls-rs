// Generated macro for remove (function)
macro_rules! Depcrate_fileremove {
() => {
// Module: crate::file
// Provides: {"remove"}
// Dependencies: {}
# [doc = " Removes a file from the filesystem."] # [doc = ""] # [doc = " # Errors"] # [doc = ""] # [doc = " This function will return an error in the following situations, but is not limited to just"] # [doc = " these cases:"] # [doc = ""] # [doc = " * The current process does not have the permission to access `path`."] # [doc = ""] # [doc = " # Example"] # [doc = " ```rust,ignore"] # [doc = " extern crate fs_extra;"] # [doc = " use fs_extra::file::remove;"] # [doc = ""] # [doc = " remove(\"foo.txt\" )?; // Remove foo.txt"] # [doc = ""] # [doc = " ```"] pub fn remove < P > (path : P) -> Result < () > where P : AsRef < Path > , { if path . as_ref () . exists () { Ok (remove_file (path) ?) } else { Ok (()) } }
};
}
