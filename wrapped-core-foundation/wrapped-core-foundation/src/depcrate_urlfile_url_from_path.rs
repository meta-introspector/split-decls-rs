// Generated macro for file_url_from_path (function)
macro_rules! Depcrate_urlfile_url_from_path {
() => {
// Module: crate::url
// Provides: {"file_url_from_path"}
// Dependencies: {}
# [test] fn file_url_from_path () { let path = "/usr/local/foo/" ; let cfstr_path = CFString :: from_static_string (path) ; let cfurl = CFURL :: from_file_system_path (cfstr_path , kCFURLPOSIXPathStyle , true) ; assert_eq ! (cfurl . get_string () . to_string () , "file:///usr/local/foo/") ; }
};
}
