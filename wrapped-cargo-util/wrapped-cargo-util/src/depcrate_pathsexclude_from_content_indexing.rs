// Generated macro for exclude_from_content_indexing (function)
macro_rules! Depcrate_pathsexclude_from_content_indexing {
() => {
// Module: crate::paths
// Provides: {"exclude_from_content_indexing"}
// Dependencies: {}
# [doc = " Marks the directory as excluded from content indexing."] # [doc = ""] # [doc = " This is recommended to prevent the content of derived/temporary files from being indexed."] # [doc = " This is very important for Windows users, as the live content indexing significantly slows"] # [doc = " cargo's I/O operations."] # [doc = ""] # [doc = " This is currently a no-op on non-Windows platforms."] fn exclude_from_content_indexing (path : & Path) { # [cfg (windows)] { use std :: iter :: once ; use std :: os :: windows :: prelude :: OsStrExt ; use windows_sys :: Win32 :: Storage :: FileSystem :: { FILE_ATTRIBUTE_NOT_CONTENT_INDEXED , GetFileAttributesW , SetFileAttributesW , } ; let path : Vec < u16 > = path . as_os_str () . encode_wide () . chain (once (0)) . collect () ; unsafe { SetFileAttributesW (path . as_ptr () , GetFileAttributesW (path . as_ptr ()) | FILE_ATTRIBUTE_NOT_CONTENT_INDEXED ,) ; } } # [cfg (not (windows))] { let _ = path ; } }
};
}
