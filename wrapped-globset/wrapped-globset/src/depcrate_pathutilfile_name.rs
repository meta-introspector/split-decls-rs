// Generated macro for file_name (function)
macro_rules! Depcrate_pathutilfile_name {
() => {
// Module: crate::pathutil
// Provides: {"file_name"}
// Dependencies: {}
# [doc = " The final component of the path, if it is a normal file."] # [doc = ""] # [doc = " If the path terminates in `..`, or consists solely of a root of prefix,"] # [doc = " file_name will return `None`."] pub (crate) fn file_name < 'a > (path : & Cow < 'a , [u8] >) -> Option < Cow < 'a , [u8] > > { if path . is_empty () { return None ; } let last_slash = path . rfind_byte (b'/') . map (| i | i + 1) . unwrap_or (0) ; let got = match * path { Cow :: Borrowed (path) => Cow :: Borrowed (& path [last_slash ..]) , Cow :: Owned (ref path) => { let mut path = path . clone () ; path . drain_bytes (.. last_slash) ; Cow :: Owned (path) } } ; if got == & b".." [..] { return None ; } Some (got) }
};
}
