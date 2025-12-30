// Generated macro for non_existent_bundle (function)
macro_rules! Depcrate_bundlenon_existent_bundle {
() => {
// Module: crate::bundle
// Provides: {"non_existent_bundle"}
// Dependencies: {}
# [test] fn non_existent_bundle () { use crate :: string :: CFString ; use crate :: url :: { kCFURLPOSIXPathStyle , CFURL } ; let cfstr_path = CFString :: from_static_string ("/usr/local/foo") ; let cfurl_path = CFURL :: from_file_system_path (cfstr_path , kCFURLPOSIXPathStyle , true) ; assert ! (CFBundle :: new (cfurl_path) . is_none ()) ; }
};
}
