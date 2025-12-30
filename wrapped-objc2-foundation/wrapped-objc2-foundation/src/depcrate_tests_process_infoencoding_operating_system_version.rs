// Generated macro for encoding_operating_system_version (function)
macro_rules! Depcrate_tests_process_infoencoding_operating_system_version {
() => {
// Module: crate::tests::process_info
// Provides: {"encoding_operating_system_version"}
// Dependencies: {}
# [test] # [cfg (not (feature = "gnustep-1-7"))] fn encoding_operating_system_version () { let info = NSProcessInfo :: processInfo () ; let _version = info . operatingSystemVersion () ; }
};
}
