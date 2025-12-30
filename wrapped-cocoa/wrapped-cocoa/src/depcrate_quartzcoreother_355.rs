// Generated macro for other_355 (other)
macro_rules! Depcrate_quartzcoreother_355 {
() => {
// Module: crate::quartzcore
// Provides: {"other_355"}
// Dependencies: {}
# [cfg_attr (feature = "link" , link (name = "QuartzCore" , kind = "framework"))] extern "C" { static kCARendererColorSpace : CFStringRef ; static kCARendererMetalCommandQueue : CFStringRef ; fn CACurrentMediaTime () -> CFTimeInterval ; fn CATransform3DIsIdentity (t : CATransform3D) -> bool ; fn CATransform3DEqualToTransform (a : CATransform3D , b : CATransform3D) -> bool ; fn CATransform3DMakeTranslation (tx : CGFloat , ty : CGFloat , tz : CGFloat) -> CATransform3D ; fn CATransform3DMakeScale (sx : CGFloat , sy : CGFloat , sz : CGFloat) -> CATransform3D ; fn CATransform3DMakeRotation (angle : CGFloat , x : CGFloat , y : CGFloat , z : CGFloat ,) -> CATransform3D ; fn CATransform3DTranslate (t : CATransform3D , tx : CGFloat , ty : CGFloat , tz : CGFloat ,) -> CATransform3D ; fn CATransform3DScale (t : CATransform3D , sx : CGFloat , sy : CGFloat , sz : CGFloat) -> CATransform3D ; fn CATransform3DRotate (t : CATransform3D , angle : CGFloat , x : CGFloat , y : CGFloat , z : CGFloat ,) -> CATransform3D ; fn CATransform3DConcat (a : CATransform3D , b : CATransform3D) -> CATransform3D ; fn CATransform3DInvert (t : CATransform3D) -> CATransform3D ; fn CATransform3DMakeAffineTransform (m : CGAffineTransform) -> CATransform3D ; fn CATransform3DIsAffine (t : CATransform3D) -> bool ; fn CATransform3DGetAffineTransform (t : CATransform3D) -> CGAffineTransform ; }
};
}
