// Generated macro for path_equals (function)
macro_rules! Depcrate_walkpath_equals {
() => {
// Module: crate::walk
// Provides: {"path_equals"}
// Dependencies: {}
# [doc = " Returns true if and only if the given directory entry is believed to be"] # [doc = " equivalent to the given handle. If there was a problem querying the path"] # [doc = " for information to determine equality, then that error is returned."] fn path_equals (dent : & DirEntry , handle : & Handle) -> Result < bool , Error > { # [cfg (unix)] fn never_equal (dent : & DirEntry , handle : & Handle) -> bool { dent . ino () != Some (handle . ino ()) } # [cfg (not (unix))] fn never_equal (_ : & DirEntry , _ : & Handle) -> bool { false } if dent . is_stdin () || never_equal (dent , handle) { return Ok (false) ; } Handle :: from_path (dent . path ()) . map (| h | & h == handle) . map_err (| err | Error :: Io (err) . with_path (dent . path ())) }
};
}
