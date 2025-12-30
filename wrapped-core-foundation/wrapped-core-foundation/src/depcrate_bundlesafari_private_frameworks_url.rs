// Generated macro for safari_private_frameworks_url (function)
macro_rules! Depcrate_bundlesafari_private_frameworks_url {
() => {
// Module: crate::bundle
// Provides: {"safari_private_frameworks_url"}
// Dependencies: {}
# [test] fn safari_private_frameworks_url () { use crate :: string :: CFString ; use crate :: url :: { kCFURLPOSIXPathStyle , CFURL } ; let cfstr_path = CFString :: from_static_string ("/Applications/Safari.app") ; let cfurl_path = CFURL :: from_file_system_path (cfstr_path , kCFURLPOSIXPathStyle , true) ; let cfurl_executable = CFBundle :: new (cfurl_path) . expect ("Safari not present") . private_frameworks_url () ; assert ! (cfurl_executable . is_some ()) ; assert_eq ! (cfurl_executable . unwrap () . absolute () . get_file_system_path (kCFURLPOSIXPathStyle) . to_string () , "/Applications/Safari.app/Contents/Frameworks") ; }
};
}
