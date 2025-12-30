// Generated macro for tests (module)
macro_rules! Depcrate_geometrytests {
() => {
// Module: crate::geometry
// Provides: {"tests"}
// Dependencies: {}
# [cfg (test)] mod tests { # [test] # [cfg (any (all (target_vendor = "apple" , target_os = "macos") , feature = "gnustep-1-7"))] # [cfg (feature = "objc2-core-foundation")] fn test_partial_eq () { use super :: * ; use crate :: { NSEqualPoints , NSEqualRects , NSEqualSizes } ; use objc2_core_foundation :: CGFloat ; let cases : & [(CGFloat , CGFloat)] = & [(0.0 , 0.0) , (- 0.0 , - 0.0) , (0.0 , - 0.0) , (1.0 , 1.0 + CGFloat :: EPSILON) , (0.0 , CGFloat :: MIN_POSITIVE) , (0.0 , CGFloat :: EPSILON) , (1.0 , 1.0) , (1.0 , - 1.0) , (CGFloat :: INFINITY , CGFloat :: INFINITY) , (CGFloat :: INFINITY , CGFloat :: NEG_INFINITY) , (CGFloat :: NEG_INFINITY , CGFloat :: NEG_INFINITY) , (CGFloat :: NAN , 0.0) , (CGFloat :: NAN , 1.0) , (CGFloat :: NAN , CGFloat :: NAN) , (CGFloat :: NAN , - CGFloat :: NAN) , (- CGFloat :: NAN , - CGFloat :: NAN) , (CGFloat :: NAN , CGFloat :: INFINITY) ,] ; for case in cases { let point_a = NSPoint :: new (case . 0 , case . 1) ; let point_b = NSPoint :: new (case . 0 , case . 1) ; let actual = NSEqualPoints (point_a , point_b) ; assert_eq ! (point_a == point_b , actual) ; if case . 0 >= 0.0 && case . 1 >= 0.0 { let size_a = NSSize :: new (case . 0 , case . 1) ; let size_b = NSSize :: new (case . 0 , case . 1) ; let actual = NSEqualSizes (size_a , size_b) ; assert_eq ! (size_a == size_b , actual) ; let rect_a = NSRect :: new (point_a , size_a) ; let rect_b = NSRect :: new (point_b , size_b) ; let actual = NSEqualRects (rect_a , rect_b) ; assert_eq ! (rect_a == rect_b , actual) ; } } } }
};
}
