// Generated macro for ffi (module)
macro_rules! Depcrate_geometryffi {
() => {
// Module: crate::geometry
// Provides: {"ffi"}
// Dependencies: {}
mod ffi { use crate :: base :: { boolean_t , CGFloat } ; use crate :: geometry :: { CGAffineTransform , CGPoint , CGRect , CGSize } ; use core_foundation :: dictionary :: CFDictionaryRef ; # [cfg_attr (feature = "link" , link (name = "CoreGraphics" , kind = "framework"))] extern "C" { pub fn CGRectInset (rect : CGRect , dx : CGFloat , dy : CGFloat) -> CGRect ; pub fn CGRectMakeWithDictionaryRepresentation (dict : CFDictionaryRef , rect : * mut CGRect ,) -> boolean_t ; pub fn CGRectIsEmpty (rect : CGRect) -> boolean_t ; pub fn CGRectIntersectsRect (rect1 : CGRect , rect2 : CGRect) -> boolean_t ; pub fn CGAffineTransformInvert (t : CGAffineTransform) -> CGAffineTransform ; pub fn CGPointApplyAffineTransform (point : CGPoint , t : CGAffineTransform) -> CGPoint ; pub fn CGRectApplyAffineTransform (rect : CGRect , t : CGAffineTransform) -> CGRect ; pub fn CGSizeApplyAffineTransform (size : CGSize , t : CGAffineTransform) -> CGSize ; pub fn CGRectContainsPoint (rect : CGRect , point : CGPoint) -> boolean_t ; } }
};
}
