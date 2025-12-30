// Generated macro for AffinePoint (enum)
macro_rules! Depcrate_devAffinePoint {
() => {
// Module: crate::dev
// Provides: {"AffinePoint"}
// Dependencies: {}
# [doc = " Example affine point type"] # [derive (Clone , Copy , Default , Debug , Eq , PartialEq)] pub enum AffinePoint { # [doc = " Result of fixed-based scalar multiplication."] FixedBaseOutput (Scalar) , # [doc = " Identity."] # [default] Identity , # [doc = " Base point."] Generator , # [doc = " Point corresponding to a given [`EncodedPoint`]."] Other (EncodedPoint) , }
};
}
