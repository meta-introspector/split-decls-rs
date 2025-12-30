// Generated macro for absolute_file_url (function)
macro_rules! Depcrate_urlabsolute_file_url {
() => {
// Module: crate::url
// Provides: {"absolute_file_url"}
// Dependencies: {}
# [test] fn absolute_file_url () { use core_foundation_sys :: url :: CFURLCreateWithFileSystemPathRelativeToBase ; use std :: path :: PathBuf ; let path = "/usr/local/foo" ; let file = "bar" ; let cfstr_path = CFString :: from_static_string (path) ; let cfstr_file = CFString :: from_static_string (file) ; let cfurl_base = CFURL :: from_file_system_path (cfstr_path , kCFURLPOSIXPathStyle , true) ; let cfurl_relative : CFURL = unsafe { let url_ref = CFURLCreateWithFileSystemPathRelativeToBase (kCFAllocatorDefault , cfstr_file . as_concrete_TypeRef () , kCFURLPOSIXPathStyle , false as u8 , cfurl_base . as_concrete_TypeRef () ,) ; TCFType :: wrap_under_create_rule (url_ref) } ; let mut absolute_path = PathBuf :: from (path) ; absolute_path . push (file) ; assert_eq ! (cfurl_relative . get_file_system_path (kCFURLPOSIXPathStyle) . to_string () , file) ; assert_eq ! (cfurl_relative . absolute () . get_file_system_path (kCFURLPOSIXPathStyle) . to_string () , absolute_path . to_str () . unwrap ()) ; }
};
}
