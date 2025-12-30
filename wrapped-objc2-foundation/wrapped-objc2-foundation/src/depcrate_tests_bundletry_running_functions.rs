// Generated macro for try_running_functions (function)
macro_rules! Depcrate_tests_bundletry_running_functions {
() => {
// Module: crate::tests::bundle
// Provides: {"try_running_functions"}
// Dependencies: {}
# [test] # [cfg (feature = "NSString")] # [cfg (feature = "NSDictionary")] # [cfg_attr (not (target_os = "macos") , ignore = "varies between platforms")] fn try_running_functions () { let bundle = NSBundle :: mainBundle () ; let _ = bundle . description () ; assert_eq ! (format ! ("{:?}" , bundle . infoDictionary () . unwrap ()) , "{}") ; assert_eq ! (bundle . name () , None) ; }
};
}
