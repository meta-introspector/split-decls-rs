// Generated macro for nsrect (function)
macro_rules! Depcrate_tests_valuensrect {
() => {
// Module: crate::tests::value
// Provides: {"nsrect"}
// Dependencies: {}
# [test] # [cfg (all (feature = "NSGeometry" , feature = "objc2-core-foundation"))] fn nsrect () { use crate :: { NSPoint , NSRect , NSSize } ; let rect = NSRect :: new (NSPoint :: new (1.0 , 2.0) , NSSize :: new (3.0 , 4.0)) ; let val = NSValue :: new (rect) ; assert_eq ! (val . get_rect () , Some (rect)) ; # [cfg (not (feature = "gnustep-1-7"))] assert_eq ! (unsafe { val . get ::< NSRect > () } , rect) ; }
};
}
