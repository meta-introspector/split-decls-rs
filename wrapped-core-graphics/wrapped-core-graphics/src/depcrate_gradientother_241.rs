// Generated macro for other_241 (other)
macro_rules! Depcrate_gradientother_241 {
() => {
// Module: crate::gradient
// Provides: {"other_241"}
// Dependencies: {}
# [cfg_attr (feature = "link" , link (name = "CoreGraphics" , kind = "framework"))] extern "C" { fn CGGradientCreateWithColorComponents (color_space : crate :: sys :: CGColorSpaceRef , components : * const CGFloat , locations : * const CGFloat , count : usize ,) -> crate :: sys :: CGGradientRef ; fn CGGradientCreateWithColors (color_space : crate :: sys :: CGColorSpaceRef , colors : CFArrayRef , locations : * const CGFloat ,) -> crate :: sys :: CGGradientRef ; }
};
}
