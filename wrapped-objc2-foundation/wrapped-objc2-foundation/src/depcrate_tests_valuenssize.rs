// Generated macro for nssize (function)
macro_rules! Depcrate_tests_valuenssize {
() => {
// Module: crate::tests::value
// Provides: {"nssize"}
// Dependencies: {}
# [test] # [cfg (all (feature = "NSGeometry" , feature = "objc2-core-foundation"))] fn nssize () { use crate :: NSSize ; let point = NSSize :: new (1.0 , 2.0) ; let val = NSValue :: new (point) ; assert_eq ! (val . get_size () , Some (point)) ; # [cfg (not (feature = "gnustep-1-7"))] assert_eq ! (unsafe { val . get ::< NSSize > () } , point) ; }
};
}
