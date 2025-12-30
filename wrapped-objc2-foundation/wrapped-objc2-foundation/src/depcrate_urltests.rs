// Generated macro for tests (module)
macro_rules! Depcrate_urltests {
() => {
// Module: crate::url
// Provides: {"tests"}
// Dependencies: {}
# [cfg (test)] # [cfg (unix)] mod tests { use std :: os :: unix :: ffi :: OsStrExt ; use super :: * ; # [test] fn invalid_path () { assert_eq ! (NSURL :: from_file_path ("") , None) ; assert_eq ! (NSURL :: from_file_path ("/\0/a") , None) ; } # [test] fn roundtrip () { let path = Path :: new (OsStr :: from_bytes (b"/abc/def")) ; let url = NSURL :: from_file_path (path) . unwrap () ; assert_eq ! (url . to_file_path () . unwrap () , path) ; let path = Path :: new (OsStr :: from_bytes (b"/\x08")) ; let url = NSURL :: from_file_path (path) . unwrap () ; assert_eq ! (url . to_file_path () . unwrap () , path) ; let path = Path :: new (OsStr :: from_bytes (b"/\x08")) ; let url = NSURL :: from_file_path (path) . unwrap () ; assert_eq ! (url . to_file_path () . unwrap () , path) ; } # [test] # [cfg (all (feature = "NSData" , feature = "NSFileManager" , feature = "NSError"))] # [ignore = "needs HFS+ file system"] fn special_paths () { use crate :: { NSData , NSFileManager } ; let manager = NSFileManager :: defaultManager () ; let path = Path :: new (OsStr :: from_bytes (b"\xf8")) ; let url = NSURL :: from_file_path ("%F8") . unwrap () ; std :: fs :: write (path , "") . unwrap () ; assert_eq ! (NSData :: dataWithContentsOfURL (& url) , Some (NSData :: new ())) ; manager . removeItemAtURL_error (& url) . unwrap () ; } }
};
}
