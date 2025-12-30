// Generated macro for impl_28 (impl)
macro_rules! Depcrate_geometryimpl_28 {
() => {
// Module: crate::geometry
// Provides: {"impl_28"}
// Dependencies: {}
impl CGSize { # [inline] pub fn new (width : CGFloat , height : CGFloat) -> CGSize { CGSize { width , height } } # [inline] pub fn apply_transform (& self , t : & CGAffineTransform) -> CGSize { unsafe { ffi :: CGSizeApplyAffineTransform (* self , * t) } } }
};
}
