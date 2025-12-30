// Generated macro for impl_34 (impl)
macro_rules! Depcrate_geometryimpl_34 {
() => {
// Module: crate::geometry
// Provides: {"impl_34"}
// Dependencies: {}
impl CGAffineTransform { # [inline] pub fn new (a : CGFloat , b : CGFloat , c : CGFloat , d : CGFloat , tx : CGFloat , ty : CGFloat ,) -> CGAffineTransform { CGAffineTransform { a , b , c , d , tx , ty } } # [inline] pub fn invert (& self) -> CGAffineTransform { unsafe { ffi :: CGAffineTransformInvert (* self) } } }
};
}
