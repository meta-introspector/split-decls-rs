// Generated macro for other_38 (other)
macro_rules! Depcrate_colorother_38 {
() => {
// Module: crate::color
// Provides: {"other_38"}
// Dependencies: {}
# [cfg_attr (feature = "link" , link (name = "CoreGraphics" , kind = "framework"))] extern "C" { fn CGColorCreateGenericRGB (red : CGFloat , green : CGFloat , blue : CGFloat , alpha : CGFloat ,) -> crate :: sys :: CGColorRef ; # [cfg (feature = "catalina")] fn CGColorCreateSRGB (red : CGFloat , green : CGFloat , blue : CGFloat , alpha : CGFloat ,) -> crate :: sys :: CGColorRef ; fn CGColorGetTypeID () -> CFTypeID ; }
};
}
