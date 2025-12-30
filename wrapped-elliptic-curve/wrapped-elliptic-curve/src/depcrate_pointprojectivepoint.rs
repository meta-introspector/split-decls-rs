// Generated macro for ProjectivePoint (type)
macro_rules! Depcrate_pointProjectivePoint {
() => {
// Module: crate::point
// Provides: {"ProjectivePoint"}
// Dependencies: {}
# [doc = " Projective point type for a given curve with a [`CurveArithmetic`]"] # [doc = " implementation."] # [cfg (feature = "arithmetic")] pub type ProjectivePoint < C > = < C as CurveArithmetic > :: ProjectivePoint ;
};
}
