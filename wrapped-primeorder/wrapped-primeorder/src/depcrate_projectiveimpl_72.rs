// Generated macro for impl_72 (impl)
macro_rules! Depcrate_projectiveimpl_72 {
() => {
// Module: crate::projective
// Provides: {"impl_72"}
// Dependencies: {}
impl < C > ConstantTimeEq for ProjectivePoint < C > where C : PrimeCurveParams , { fn ct_eq (& self , other : & Self) -> Choice { self . to_affine () . ct_eq (& other . to_affine ()) } }
};
}
