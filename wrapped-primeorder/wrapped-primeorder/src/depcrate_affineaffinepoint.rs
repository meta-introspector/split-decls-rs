// Generated macro for AffinePoint (struct)
macro_rules! Depcrate_affineAffinePoint {
() => {
// Module: crate::affine
// Provides: {"AffinePoint"}
// Dependencies: {}
# [doc = " Point on a Weierstrass curve in affine coordinates."] # [derive (Clone , Copy , Debug)] pub struct AffinePoint < C : PrimeCurveParams > { # [doc = " x-coordinate"] pub (crate) x : C :: FieldElement , # [doc = " y-coordinate"] pub (crate) y : C :: FieldElement , # [doc = " Is this point the point at infinity? 0 = no, 1 = yes"] # [doc = ""] # [doc = " This is a proxy for [`Choice`], but uses `u8` instead to permit `const`"] # [doc = " constructors for `IDENTITY` and `GENERATOR`."] pub (crate) infinity : u8 , }
};
}
