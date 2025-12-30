// Generated macro for non_utf8 (function)
macro_rules! Depcrate_urlnon_utf8 {
() => {
// Module: crate::url
// Provides: {"non_utf8"}
// Dependencies: {}
# [cfg (unix)] # [test] fn non_utf8 () { use std :: ffi :: OsStr ; let path = Path :: new (OsStr :: from_bytes (b"/\xC0/blame")) ; let cfurl = CFURL :: from_path (path , false) . unwrap () ; assert_eq ! (cfurl . to_path () . unwrap () , path) ; let len = unsafe { CFURLGetBytes (cfurl . as_concrete_TypeRef () , ptr :: null_mut () , 0) } ; assert_eq ! (len , 17) ; }
};
}
