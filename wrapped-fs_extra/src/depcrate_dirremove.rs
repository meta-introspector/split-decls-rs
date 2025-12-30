// Generated macro for remove (function)
macro_rules! Depcrate_dirremove {
() => {
// Module: crate::dir
// Provides: {"remove"}
// Dependencies: {}
# [doc = " Removes directory."] # [doc = ""] # [doc = " # Example"] # [doc = " ```rust,ignore"] # [doc = " extern crate fs_extra;"] # [doc = " use fs_extra::dir::remove;"] # [doc = ""] # [doc = " remove(\"source/dir1\"); // remove dir1"] # [doc = " ```"] pub fn remove < P : AsRef < Path > > (path : P) -> Result < () > { if path . as_ref () . exists () { Ok (remove_dir_all (path) ?) } else { Ok (()) } }
};
}
