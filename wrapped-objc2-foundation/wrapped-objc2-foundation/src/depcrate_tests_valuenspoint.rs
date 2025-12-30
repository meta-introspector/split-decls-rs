// Generated macro for nspoint (function)
macro_rules! Depcrate_tests_valuenspoint {
() => {
// Module: crate::tests::value
// Provides: {"nspoint"}
// Dependencies: {}
# [test] # [cfg (all (feature = "NSGeometry" , feature = "objc2-core-foundation"))] fn nspoint () { use crate :: NSPoint ; let point = NSPoint :: new (1.0 , 2.0) ; let val = NSValue :: new (point) ; assert_eq ! (val . get_point () , Some (point)) ; # [cfg (not (feature = "gnustep-1-7"))] assert_eq ! (unsafe { val . get ::< NSPoint > () } , point) ; }
};
}
