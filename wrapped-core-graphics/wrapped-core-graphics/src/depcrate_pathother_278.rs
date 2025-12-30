// Generated macro for other_278 (other)
macro_rules! Depcrate_pathother_278 {
() => {
// Module: crate::path
// Provides: {"other_278"}
// Dependencies: {}
# [cfg_attr (feature = "link" , link (name = "CoreGraphics" , kind = "framework"))] extern "C" { fn CGPathCreateWithRect (rect : CGRect , transform : * const CGAffineTransform ,) -> crate :: sys :: CGPathRef ; fn CGPathApply (path : crate :: sys :: CGPathRef , info : * mut c_void , function : CGPathApplierFunction) ; fn CGPathGetTypeID () -> CFTypeID ; }
};
}
