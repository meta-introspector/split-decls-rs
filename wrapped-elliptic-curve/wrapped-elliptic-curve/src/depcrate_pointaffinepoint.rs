// Generated macro for AffinePoint (type)
macro_rules! Depcrate_pointAffinePoint {
() => {
// Module: crate::point
// Provides: {"AffinePoint"}
// Dependencies: {}
# [doc = " Affine point type for a given curve with a [`CurveArithmetic`]"] # [doc = " implementation."] # [cfg (feature = "arithmetic")] pub type AffinePoint < C > = < C as CurveArithmetic > :: AffinePoint ;
};
}
