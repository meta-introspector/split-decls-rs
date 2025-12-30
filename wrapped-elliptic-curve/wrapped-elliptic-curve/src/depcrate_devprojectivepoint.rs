// Generated macro for ProjectivePoint (enum)
macro_rules! Depcrate_devProjectivePoint {
() => {
// Module: crate::dev
// Provides: {"ProjectivePoint"}
// Dependencies: {}
# [doc = " Example projective point type"] # [derive (Clone , Copy , Default , Debug , Eq , PartialEq)] pub enum ProjectivePoint { # [doc = " Result of fixed-based scalar multiplication"] FixedBaseOutput (Scalar) , # [doc = " Is this point the identity point?"] # [default] Identity , # [doc = " Is this point the generator point?"] Generator , # [doc = " Is this point a different point corresponding to a given [`AffinePoint`]"] Other (AffinePoint) , }
};
}
