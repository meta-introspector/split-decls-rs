// Generated macro for toll_free_bridging (function)
macro_rules! Depcrate_tests_stringtoll_free_bridging {
() => {
// Module: crate::tests::string
// Provides: {"toll_free_bridging"}
// Dependencies: {}
# [test] # [cfg (feature = "objc2-core-foundation")] # [cfg (not (feature = "gnustep-1-7"))] fn toll_free_bridging () { use objc2_core_foundation :: CFString ; let string = ns_string ! ("foo") ; let cf_string : & CFString = string . as_ref () ; let _ : & NSString = cf_string . as_ref () ; assert_eq ! (cf_string . to_string () , string . to_string ()) ; }
};
}
