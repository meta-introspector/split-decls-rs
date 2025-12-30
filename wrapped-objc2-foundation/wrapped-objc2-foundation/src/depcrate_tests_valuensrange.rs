// Generated macro for nsrange (function)
macro_rules! Depcrate_tests_valuensrange {
() => {
// Module: crate::tests::value
// Provides: {"nsrange"}
// Dependencies: {}
# [test] # [cfg (feature = "NSRange")] fn nsrange () { use crate :: NSRange ; let range = NSRange :: from (1 .. 2) ; let val = NSValue :: new (range) ; assert_eq ! (val . get_range () , Some (range)) ; # [cfg (feature = "objc2-core-foundation")] { assert_eq ! (val . get_point () , None) ; assert_eq ! (val . get_size () , None) ; assert_eq ! (val . get_rect () , None) ; } # [cfg (not (feature = "gnustep-1-7"))] assert_eq ! (unsafe { val . get ::< NSRange > () } , range) ; }
};
}
