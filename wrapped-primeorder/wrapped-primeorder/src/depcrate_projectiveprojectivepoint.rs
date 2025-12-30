// Generated macro for ProjectivePoint (struct)
macro_rules! Depcrate_projectiveProjectivePoint {
() => {
// Module: crate::projective
// Provides: {"ProjectivePoint"}
// Dependencies: {}
# [doc = " Point on a Weierstrass curve in projective coordinates."] # [derive (Clone , Copy , Debug)] pub struct ProjectivePoint < C : PrimeCurveParams > { pub (crate) x : C :: FieldElement , pub (crate) y : C :: FieldElement , pub (crate) z : C :: FieldElement , }
};
}
