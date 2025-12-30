// Generated macro for impl_30 (impl)
macro_rules! Depcrate_geometryimpl_30 {
() => {
// Module: crate::geometry
// Provides: {"impl_30"}
// Dependencies: {}
impl CGPoint { # [inline] pub fn new (x : CGFloat , y : CGFloat) -> CGPoint { CGPoint { x , y } } # [inline] pub fn apply_transform (& self , t : & CGAffineTransform) -> CGPoint { unsafe { ffi :: CGPointApplyAffineTransform (* self , * t) } } }
};
}
