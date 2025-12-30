// Generated macro for file_name_ext (function)
macro_rules! Depcrate_pathutilfile_name_ext {
() => {
// Module: crate::pathutil
// Provides: {"file_name_ext"}
// Dependencies: {}
# [doc = " Return a file extension given a path's file name."] # [doc = ""] # [doc = " Note that this does NOT match the semantics of std::path::Path::extension."] # [doc = " Namely, the extension includes the `.` and matching is otherwise more"] # [doc = " liberal. Specifically, the extension is:"] # [doc = ""] # [doc = " * None, if the file name given is empty;"] # [doc = " * None, if there is no embedded `.`;"] # [doc = " * Otherwise, the portion of the file name starting with the final `.`."] # [doc = ""] # [doc = " e.g., A file name of `.rs` has an extension `.rs`."] # [doc = ""] # [doc = " N.B. This is done to make certain glob match optimizations easier. Namely,"] # [doc = " a pattern like `*.rs` is obviously trying to match files with a `rs`"] # [doc = " extension, but it also matches files like `.rs`, which doesn't have an"] # [doc = " extension according to std::path::Path::extension."] pub (crate) fn file_name_ext < 'a > (name : & Cow < 'a , [u8] > ,) -> Option < Cow < 'a , [u8] > > { if name . is_empty () { return None ; } let last_dot_at = match name . rfind_byte (b'.') { None => return None , Some (i) => i , } ; Some (match * name { Cow :: Borrowed (name) => Cow :: Borrowed (& name [last_dot_at ..]) , Cow :: Owned (ref name) => { let mut name = name . clone () ; name . drain_bytes (.. last_dot_at) ; Cow :: Owned (name) } }) }
};
}
